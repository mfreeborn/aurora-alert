use yew::prelude::*;

// const GREEN: RGBColor = RGBColor(0, 204, 0);
// const YELLOW: RGBColor = RGBColor(255, 255, 87);
// const AMBER: RGBColor = RGBColor(255, 191, 0);
// const RED: RGBColor = RGBColor(235, 0, 0);

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

    yew_hooks::use_effect_once(move || {
        p.run();
        || ()
    });

    html! {
        <div id={id.clone()} class={class.clone()}></div>
    }
}
