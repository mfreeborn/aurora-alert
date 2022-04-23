use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PlotProps {
    pub id: String,
    pub plot: plotly::Plot,
    pub class: Option<Classes>,
}

#[function_component(Plot)]
pub fn plot(props: &PlotProps) -> Html {
    let PlotProps { id, plot, class } = props;

    let p = yew_hooks::use_async::<_, _, ()>({
        let id = id.clone();
        let plot = plot.clone();
        async move {
            plotly::bindings::new_plot(&id, &plot).await;
            Ok(())
        }
    });

    {
        let id = id.clone();
        let plot = plot.clone();
        use_effect_with_deps(
            move |(_, _)| {
                p.run();
                || ()
            },
            (id, plot),
        );
    }

    html! {
        <div id={id.clone()} class={class.clone()}></div>
    }
}
