use ordered_float::OrderedFloat;
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use web_sys::HtmlCanvasElement;
use yew::prelude::*;
use yew_hooks::{use_async_with_options, UseAsyncOptions};

use crate::services::charts::{self, ActivityDataPoint};

fn draw_chart(canvas: HtmlCanvasElement, data: &charts::ActivityData) {
    let backend = CanvasBackend::with_canvas_object(canvas).expect("cannot find canvas element");
    let area = backend.into_drawing_area();

    area.fill(&WHITE).unwrap();
    let max_value = OrderedFloat(
        *data
            .activities
            .iter()
            .max_by_key(|d| d.value)
            .unwrap()
            .value,
    );

    let max_y = *if max_value < OrderedFloat(50.0) {
        OrderedFloat(50.0)
    } else if max_value < OrderedFloat(100.0) {
        OrderedFloat(100.0)
    } else if max_value < OrderedFloat(200.0) {
        OrderedFloat(200.0)
    } else {
        max_value
    };

    let mut chart = ChartBuilder::on(&area)
        .margin(20_i32)
        .x_label_area_size(130_i32)
        .y_label_area_size(30_i32)
        // note that into_segmented() adds an extra value to the axis, hance only 0..23 with 24 datapoints
        .build_cartesian_2d((0_usize..23).into_segmented(), 0_f32..max_y)
        .unwrap();

    chart
        .configure_mesh()
        .disable_y_mesh()
        .disable_x_mesh()
        .x_labels(24)
        .y_labels(3)
        // .x_label_style(
        // TextStyle::from(("sans-serif", 10).into_font()).transform(FontTransform::Rotate270),
        // )
        .x_label_formatter(&|x| {
            let i = match x {
                SegmentValue::CenterOf(i) => *i,
                _ => 0,
            };
            let datetime = &data.activities[23 - i].datetime;
            let format = if i == 0 || datetime.time() == chrono::NaiveTime::from_hms(0, 0, 0) {
                datetime.format("%-d %b %y %H:%M")
            } else {
                datetime.format("%H:%M")
            };
            format.to_string()
        })
        .draw()
        .unwrap();

    chart
        .draw_series(data.activities.iter().rev().enumerate().map(
            |(
                i,
                ActivityDataPoint {
                    datetime: _datetime,
                    value,
                },
            )| {
                let x0 = SegmentValue::Exact(i);
                let x1 = SegmentValue::Exact(i + 1);
                let mut bar = Rectangle::new(
                    [
                        // top left coord
                        (x0, **value),
                        // bottom right coord
                        (x1, 0.),
                    ],
                    RED.filled(),
                );
                bar.set_margin(0, 0, 5, 5);
                bar
            },
        ))
        .unwrap();

    area.present().unwrap();
}

#[function_component(Home)]
pub fn home() -> Html {
    log::info!("render home page");
    let canvas_ref = use_node_ref();

    let fetch_chart_data = {
        use_async_with_options(
            async move { charts::get_activity_data().await },
            UseAsyncOptions::enable_auto(),
        )
    };

    if let Some(data) = &fetch_chart_data.data {
        let canvas: HtmlCanvasElement = canvas_ref.cast().unwrap();
        draw_chart(canvas, data);
    };

    html! {
        <>
            <h1>{ "Home" }</h1>
            <canvas ref={canvas_ref} width=1500 height=500 style="height: 500px; width: 1500px;"></canvas>
        </>
    }
}
