use std::f64::consts::PI;

use chrono::Timelike;
use ordered_float::{NotNan, OrderedFloat};
use plotters::{
    coord::{
        ranged1d::SegmentedCoord,
        types::{RangedCoordf32, RangedCoordusize},
        Shift,
    },
    prelude::*,
};
use plotters_canvas::CanvasBackend;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use yew::prelude::*;
use yew_hooks::use_window_size;

use crate::services::charts::{ActivityData, ActivityDataPoint};

type DateTimeLocal = chrono::DateTime<chrono::Local>;

const GREEN: RGBColor = RGBColor(0, 204, 0);
const YELLOW: RGBColor = RGBColor(255, 255, 87);
const AMBER: RGBColor = RGBColor(255, 191, 0);
const RED: RGBColor = RGBColor(235, 0, 0);

#[derive(Properties, PartialEq)]
pub struct ActivityPlotProps {
    pub data: ActivityData,
}

#[function_component(ActivityPlot)]
pub fn activity_plot(props: &ActivityPlotProps) -> Html {
    log::debug!("render plot");
    let ActivityPlotProps { data } = props;

    let (screen_width, _screen_height) = use_window_size();
    let chart_spec = match screen_width as usize {
        0..=393 => ChartSize::Sm.get_spec(), // Google Pixel 5
        _ => ChartSize::Lg.get_spec(),       // 1440p desktop
    };

    let canvas_ref = use_node_ref();

    {
        let canvas_ref = canvas_ref.clone();
        let data = data.clone();
        let spec = chart_spec.clone();
        use_effect_with_deps(
            move |(data, _)| {
                let data = data.clone();
                let canvas: HtmlCanvasElement = canvas_ref.cast().unwrap();
                draw_chart(canvas, &data, &spec);
                || ()
            },
            (data, screen_width),
        );
    }

    html! {
        <div id="activity-chart-plot" class="row">
            <div class="col">
                <canvas
                    ref={canvas_ref}
                    width={chart_spec.width.to_string()}
                    height={chart_spec.height.to_string()}
                    style={format!(
                        "height: {}px; width: {}px;",
                        chart_spec.display_height,
                        chart_spec.display_width
                    )}
                />
            </div>
        </div>
    }
}

#[derive(Clone)]
struct ChartSpec<'a> {
    display_height: i32,
    display_width: i32,
    width: i32,
    height: i32,
    figure_margin: i32,
    margin_top: i32,
    x_label_area_size: i32,
    y_label_area_size: i32,
    x_label_y_offset: i32,
    x_axis_y_offset: i32,
    bar_width: f32,
    font_lg: FontDesc<'a>,
    // font_md: FontDesc<'a>,
    font_sm: FontDesc<'a>,
    font_xs: FontDesc<'a>,
}

impl<'a> ChartSpec<'a> {
    fn new(
        display_width: i32,
        display_height: i32,
        figure_margin: i32,
        x_label_area_size: i32,
        y_label_area_size: i32,
    ) -> Self {
        let ratio: f64 = web_sys::window().unwrap().device_pixel_ratio();
        let width = (f64::from(display_width) * ratio) as i32;
        let height = (f64::from(display_height) * ratio) as i32;

        let margin_top = (30. * ratio) as i32;

        let bar_width = (width - (figure_margin * 2 + y_label_area_size)) as f32 / 24.;
        let x_label_y_offset = (height - x_label_area_size - figure_margin) + 12;
        let x_axis_y_offset = height - figure_margin - x_label_area_size;

        let font_xs = FontDesc::new(FontFamily::SansSerif, 10. * ratio, FontStyle::Normal);
        let font_sm = font_xs.resize(14. * ratio);
        // let font_md = font_xs.resize(18. * ratio);
        let font_lg = font_xs.resize(22. * ratio);

        Self {
            display_width,
            display_height,
            width,
            height,
            figure_margin,
            margin_top,
            x_label_area_size,
            y_label_area_size,
            bar_width,
            x_label_y_offset,
            x_axis_y_offset,
            font_lg,
            // font_md,
            font_sm,
            font_xs,
        }
    }

    fn x_bar_offset(&self, i: usize) -> i32 {
        // given the index of a bar, return the x-pixel offset to it's center
        self.figure_margin
            + self.y_label_area_size
            + ((self.bar_width / 2.) * (i as f32 * 2. + 1.)) as i32
    }
}

enum ChartSize {
    Sm,
    Lg,
}

impl ChartSize {
    fn get_spec(&self) -> ChartSpec {
        match *self {
            Self::Sm => ChartSpec::new(369, 500, 12, 140, 90),
            Self::Lg => ChartSpec::new(1296, 500, 10, 60, 50),
        }
    }
}

fn draw_chart(canvas: HtmlCanvasElement, data: &ActivityData, spec: &ChartSpec) {
    log::debug!("drawing activity chart");
    // set up the root canvas onto which we will build the chart
    let backend =
        CanvasBackend::with_canvas_object(canvas.clone()).expect("cannot find canvas element");
    let area = backend.into_drawing_area();
    area.fill(&WHITE).unwrap();
    area.titled("Latest Geomagnetic Activity", spec.font_lg.clone())
        .unwrap();

    // get the largest value for geomagnetic data in the current data
    let max_value = OrderedFloat(
        *data
            .activities
            .iter()
            .max_by_key(|d| d.value)
            .unwrap()
            .value,
    );

    // calculcate the maximum y value, which is either the next higher alert level threshold, or max_value;
    // whichever is greater. We go a little higher than the actual thresholds so that we can fully see
    // the threshold lines when they are drawn on the chart.
    let max_y = if max_value < OrderedFloat(50.0) {
        OrderedFloat(50.5)
    } else if max_value < OrderedFloat(100.0) {
        OrderedFloat(101.0)
    } else if max_value < OrderedFloat(200.0) {
        OrderedFloat(205.0)
    } else {
        max_value + 5.
    };

    let data_updated_at_local = data.updated_at.with_timezone(&chrono::Local);
    // build the basic chart figure
    let mut chart = ChartBuilder::on(&area)
        .margin(spec.figure_margin)
        .margin_top(spec.margin_top)
        .caption(
            format!(
                "Last updated {}",
                data_updated_at_local.format("%-d %b %y %H:%M %Z")
            ),
            spec.font_sm.clone(),
        )
        .x_label_area_size(spec.x_label_area_size)
        .y_label_area_size(spec.y_label_area_size)
        // note that into_segmented() adds an extra value to the axis, hence only 0..23 with 24 datapoints.
        .build_cartesian_2d((0_usize..23).into_segmented(), 0_f32..*max_y)
        .unwrap();

    // format the chart area incl. axes and tick labels
    chart
        .configure_mesh()
        .disable_x_mesh()
        .disable_x_axis() // we are going to build the x axis from scratch!
        .light_line_style(TRANSPARENT)
        .y_labels(6)
        .label_style(spec.font_xs.clone())
        .y_label_formatter(&|y_tick_label| (*y_tick_label as usize).to_string())
        .x_desc(format!("Time ({})", data_updated_at_local.format("%Z")))
        .y_desc("Activity (nT)")
        .axis_desc_style(spec.font_sm.clone())
        .set_all_tick_mark_size(7.)
        .draw()
        .unwrap();

    draw_threshold_lines(&mut chart, max_value);
    draw_bars(&mut chart, &data.activities);
    draw_x_axis(&area, spec);
    draw_x_labels(&canvas, &area, &data.activities, spec);

    area.present().unwrap();
}

fn value_to_colour(value: &NotNan<f32>) -> RGBColor {
    if value < &NotNan::new(50.).unwrap() {
        GREEN
    } else if value < &NotNan::new(100.).unwrap() {
        YELLOW
    } else if value < &NotNan::new(200.).unwrap() {
        AMBER
    } else {
        RED
    }
}

fn draw_x_labels(
    canvas: &HtmlCanvasElement,
    area: &DrawingArea<CanvasBackend, Shift>,
    x_data: &[ActivityDataPoint; 24],
    spec: &ChartSpec,
) {
    let raw_context: CanvasRenderingContext2d = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into()
        .ok()
        .unwrap();
    let font = &spec.font_xs;
    raw_context.set_font(&format!(
        "{} {}px {}",
        font.get_style().as_str(),
        font.get_size(),
        font.get_family().as_str()
    ));
    raw_context.set_fill_style(&"black".into());

    // always label the first bar
    let y_tick_offset = spec.x_axis_y_offset;
    let y_label_offset = spec.x_label_y_offset;
    let first_x_bar_offset = spec.x_bar_offset(0);
    draw_x_tick(area, first_x_bar_offset, y_tick_offset);
    let first_datetime = &x_data[23].datetime.with_timezone(&chrono::Local);
    let draw_first_date = first_datetime.time().hour() < 22;
    draw_x_label(
        &raw_context,
        first_datetime,
        first_x_bar_offset,
        y_label_offset,
        draw_first_date,
    );

    if spec.display_width == 369 {
        // Pixel 5
        for i in (3_usize..24).step_by(3).chain([23]) {
            let x_offset = spec.x_bar_offset(i);
            let datetime = &x_data[23 - i].datetime.with_timezone(&chrono::Local);
            draw_x_tick(area, x_offset, y_tick_offset);
            draw_x_label(
                &raw_context,
                datetime,
                x_offset,
                y_label_offset,
                datetime.time().hour() <= 2,
            );
        }
    } else if spec.display_width == 1296 {
        // 1440p, draw all the labels
        x_data.iter().rev().enumerate().skip(1).for_each(
            |(
                i,
                ActivityDataPoint {
                    datetime,
                    value: _value,
                },
            )| {
                let x_offset = spec.x_bar_offset(i);
                let datetime = &datetime.with_timezone(&chrono::Local);
                draw_x_tick(area, x_offset, y_tick_offset);
                draw_x_label(
                    &raw_context,
                    datetime,
                    x_offset,
                    y_label_offset,
                    datetime.time().hour() == 0,
                );
            },
        )
    }
}

fn draw_threshold_lines(
    chart: &mut ChartContext<
        CanvasBackend,
        Cartesian2d<SegmentedCoord<RangedCoordusize>, RangedCoordf32>,
    >,
    max_value: OrderedFloat<f32>,
) {
    [
        (OrderedFloat(0.), 50., YELLOW),
        (OrderedFloat(50.), 100., AMBER),
        (OrderedFloat(100.), 200., RED),
    ]
    .into_iter()
    .for_each(|(prev_threshold, threshold, colour)| {
        if max_value >= prev_threshold {
            chart
                .draw_series(plotters::prelude::LineSeries::new(
                    vec![
                        (SegmentValue::Exact(0), threshold),
                        (SegmentValue::Last, threshold),
                    ],
                    &colour,
                ))
                .unwrap();
        }
    });
}

fn draw_bars(
    chart: &mut ChartContext<
        CanvasBackend,
        Cartesian2d<SegmentedCoord<RangedCoordusize>, RangedCoordf32>,
    >,
    activities: &[ActivityDataPoint; 24],
) {
    chart
        .draw_series(activities.iter().rev().enumerate().map(
            |(
                i,
                ActivityDataPoint {
                    datetime: _datetime,
                    value,
                },
            )| {
                let colour = value_to_colour(value);
                let x0 = SegmentValue::Exact(i);
                let x1 = SegmentValue::Exact(i + 1);

                let mut bar = Rectangle::new(
                    [
                        // top left coord
                        (x0, **value),
                        // bottom right coord
                        (x1, 0.),
                    ],
                    colour.filled(),
                );
                bar.set_margin(0, 0, 1, 1);
                bar
            },
        ))
        .unwrap();
}

fn draw_x_axis(area: &DrawingArea<CanvasBackend, Shift>, spec: &ChartSpec) {
    area.draw(&PathElement::new(
        vec![
            (
                spec.figure_margin + spec.y_label_area_size,
                spec.height - spec.figure_margin - spec.x_label_area_size,
            ),
            (
                spec.width - spec.figure_margin,
                spec.height - spec.figure_margin - spec.x_label_area_size,
            ),
        ],
        ShapeStyle {
            color: RGBColor(0, 0, 0).to_rgba(),
            filled: true,
            stroke_width: 1,
        },
    ))
    .unwrap();
}

fn draw_x_tick(area: &DrawingArea<CanvasBackend, Shift>, x: i32, y: i32) {
    // draw a standard x axis tick at the given coordinates
    area.draw(&PathElement::new(
        vec![(x, y), (x, y + 7)],
        ShapeStyle {
            color: RGBColor(0, 0, 0).to_rgba(),
            filled: true,
            stroke_width: 1,
        },
    ))
    .unwrap();
}

fn draw_x_label(
    area: &CanvasRenderingContext2d,
    label: &DateTimeLocal,
    x: i32,
    y: i32,
    include_date: bool,
) {
    // draw a label on the x-axis at a given position. The coordinate provided represents where the end of the
    // label text will be located, that is to say, the label is drawn with VPos::Middle, HPos::End

    // just make the hardcoded constants explicit
    const LABEL_ANGLE: f64 = 325.;
    const LINE_SPACING: f64 = 1.5;

    // start by saving the current canvas context, so that we can restore it after doing all the
    // translations/rotations
    area.save();

    area.translate(f64::from(x), f64::from(y)).unwrap();

    area.set_text_baseline("middle");
    area.set_text_align("end");

    area.rotate(LABEL_ANGLE * (PI / 180.)).unwrap();

    let time_label_text = label.format("%H:%M").to_string();
    area.fill_text(&time_label_text, 0., 0.).unwrap();

    if include_date {
        // draw the date as well
        let date_label_text = label.format("%-d %b").to_string();
        let time_label_text_metrics = area.measure_text(&time_label_text).unwrap();
        let centered_x_offset = -(time_label_text_metrics.width() / 2.);
        let line_height = time_label_text_metrics.actual_bounding_box_ascent()
            + time_label_text_metrics.actual_bounding_box_descent();
        area.translate(centered_x_offset, line_height * LINE_SPACING)
            .unwrap();
        area.set_text_align("center");
        area.fill_text(&date_label_text, 0., 0.).unwrap();
    }
    area.restore();
}
