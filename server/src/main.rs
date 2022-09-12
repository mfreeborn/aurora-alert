use std::fmt::{Debug, Display};

use server::{
    configuration::get_configuration,
    startup::Application,
    tasks::{alert_task, clear_unverified_users_task, update_activity_data_task},
    telemetry::init_tracing,
};
use tokio::task::JoinError;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = get_configuration()?;
    init_tracing("debug");

    let application = Application::build(config.clone())?;
    let sock_addr = application.sock_addr;
    let application_task = tokio::spawn(application.run_until_stopped());

    // Background tasks TODO: these should be spawned once globally, rather than every time an application is spun up
    let alert_worker = tokio::spawn(alert_task(config.clone()));
    let update_activity_data_worker = tokio::spawn(update_activity_data_task(config.clone()));
    let clear_unverified_users_worker = tokio::spawn(clear_unverified_users_task(config.clone()));

    tracing::info!("running on http://{sock_addr}");

    tokio::select! {
        o = application_task => report_exit("API", o),
        o = alert_worker => report_exit("Alert task", o),
        o = update_activity_data_worker => report_exit("Update activity data task", o),
        o = clear_unverified_users_worker => report_exit("Clear unverified users task", o),
    };

    Ok(())
}

fn report_exit(task_name: &str, outcome: Result<Result<(), impl Debug + Display>, JoinError>) {
    match outcome {
        Ok(Ok(())) => {
            tracing::info!("{} has exited", task_name)
        }
        Ok(Err(e)) => {
            tracing::error!(
                error.cause_chain = ?e,
                error.message = %e,
                "{} failed",
                task_name
            )
        }
        Err(e) => {
            tracing::error!(
                error.cause_chain = ?e,
                error.message = %e,
                "{}' task failed to complete",
                task_name
            )
        }
    }
}
