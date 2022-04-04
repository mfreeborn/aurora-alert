use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use web_sys::HtmlCanvasElement;
use yew::prelude::*;

fn draw_chart(canvas: HtmlCanvasElement) {
    let backend = CanvasBackend::with_canvas_object(canvas).expect("cannot find canvas element");
    let root = backend.into_drawing_area();
    let font: FontDesc = ("sans-serif", 20.0).into();

    root.fill(&WHITE).unwrap();

    let power = 4_i32;
    let mut chart = ChartBuilder::on(&root)
        .margin(20_i32)
        .x_label_area_size(30_i32)
        .y_label_area_size(30_i32)
        .build_cartesian_2d(-1f32..1f32, -1.2f32..1.2f32)
        .unwrap();

    chart
        .configure_mesh()
        .x_labels(3)
        .y_labels(3)
        .draw()
        .unwrap();

    chart
        .draw_series(LineSeries::new(
            (-50..=50)
                .map(|x| x as f32 / 50.0)
                .map(|x| (x, x.powf(power as f32))),
            &RED,
        ))
        .unwrap();

    root.present().unwrap();
}

#[function_component(Home)]
pub fn home() -> Html {
    log::info!("render home page");
    let canvas_ref = use_node_ref();

    {
        let canvas_ref = canvas_ref.clone();
        use_effect(move || {
            //log::info!("in callback");
            //log::info!("{canvas_ref:?}");
            let canvas: HtmlCanvasElement = canvas_ref.cast().unwrap();
            //log::info!("{canvas:?}");
            draw_chart(canvas);
            || ()
        });
    }

    html! {
        <>
            <h1>{ "Home" }</h1>
            <canvas ref={canvas_ref} width=500 height=500 style="height: 500px; width: 500px;"></canvas>
        </>
    }
}
