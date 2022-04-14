use yew::prelude::*;

use crate::components::chart::ActivityChart;
use crate::routes::LinkRegister;
use crate::themes::Theme;

#[function_component(Home)]
pub fn home() -> Html {
    log::debug!("render home page");
    let theme = use_context::<Theme>().expect("Theme context not found.");

    html! {
        <>
            <div class="row">
                <div class="col">
                    <ActivityChart />
                </div>
            </div>
            <hr />
            <div class="row justify-content-center">
                <div class="col-8">
                    <p>
                        {"The chart above shows the most recent geomagnetic activity recorded by stations in the UK, run by a group at Lancaster University's Department of Physics. The magnitude of geomagnetic activity is colour-coded to give a sense of how likely it is that an aurora will be visible from the UK, weather-permitting. To subscribe for email alerts, head over to the "}
                        <LinkRegister text="Register" />
                        {" page for more information."}
                    </p>
                </div>
            </div>
            <br />
            <div class="row justify-content-center">
                <div class="col-10">
                    <p>{"An explanation of the different colours can be found below, and is directly sourced from the AuroraWatch UK group:"}</p>
                    <table class={classes!("table", "table-striped", theme.global.table_class)}>
                        <thead>
                            <tr>
                                <th>{"Colour"}</th>
                                <th>{"Description"}</th>
                                <th>{"Meaning"}</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr>
                                <th>{"Green"}</th>
                                <td>{"No significant activity"}</td>
                                <td>{"Auroras are unlikely to be visible by eye or camera from anywhere in the UK."}</td>
                            </tr>
                            <tr>
                                <th>{"Yellow"}</th>
                                <td>{"Minor geomagnetic activity"}</td>
                                <td>{"Aurora may be visible by eye from Scotland and may be visible by camera from Scotland, northern England and
                                    Northern Ireland."}</td>
                            </tr>
                            <tr>
                                <th>{"Amber"}</th>
                                <td>{"Possible aurora"}</td>
                                <td>{"Aurora is likely to be visible by eye from Scotland, northern England and Northern Ireland; possible visible
                                    from elsewhere in the UK. Photographs of the aurora are likely from anywhere in the UK."}</td>
                            </tr>
                            <tr>
                                <th>{"Red"}</th>
                                <td>{"Aurora likely"}</td>
                                <td>{"It is likely that the aurora will be visible by eye and camera from anywhere in the UK."}</td>
                            </tr>
                        </tbody>
                    </table>
                </div>
            </div>
        </>
    }
}
