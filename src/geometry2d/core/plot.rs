use super::{point, Circle as GCircle, DirectedArc, Point, Polygon, Pose, Segment};
use sity::*;

use plotters::prelude::*;

// pub trait Plot {
//     fn draw(
//         &self,
//         chart: &mut ChartContext<
//             '_,
//             BitMapBackend<'_>,
//             Cartesian2d<
//                 plotters::coord::types::RangedCoordf64,
//                 plotters::coord::types::RangedCoordf64,
//             >,
//         >,
//         color: RGBColor,
//     );
// }

//-------------------------------------------------- Point --------------------------------------------------

impl<T> Point<T>
where
    T: Number,
    T: HasValue<Output = f64>,
{
    pub fn draw(
        &self,
        chart: &mut ChartContext<
            '_,
            BitMapBackend<'_>,
            Cartesian2d<
                plotters::coord::types::RangedCoordf64,
                plotters::coord::types::RangedCoordf64,
            >,
        >,
        color: RGBColor,
        size: i32,
    ) {
        let point = self.to_value();
        let x = point.x;
        let y = point.y;

        chart
            .draw_series(std::iter::once(Circle::new(
                (x, y),
                size,
                Into::<ShapeStyle>::into(&color).filled(),
            )))
            .unwrap();
    }
}

//-------------------------------------------------- Circle --------------------------------------------------

impl<T> GCircle<T>
where
    T: Number,
    T: HasValue<Output = f64>,
{
    pub fn draw<S: Into<ShapeStyle>>(
        &self,
        chart: &mut ChartContext<
            '_,
            BitMapBackend<'_>,
            Cartesian2d<
                plotters::coord::types::RangedCoordf64,
                plotters::coord::types::RangedCoordf64,
            >,
        >,
        style: S,
    ) {
        let circle = self.to_value();
        let nb_points = 1000;

        let points: Vec<_> = (0..=nb_points)
            .map(|x| {
                let rad = (x as f64) * 2.0 * std::f64::consts::PI / (nb_points as f64);
                (
                    circle.center().x + circle.radius() * rad.cos(),
                    circle.center().y + circle.radius() * rad.sin(),
                )
            })
            .collect();

        chart.draw_series(LineSeries::new(points, style)).unwrap();
    }
}

//-------------------------------------------------- DirectedArc --------------------------------------------------

impl<T> DirectedArc<T>
where
    T: Number,
    T: HasValue<Output = f64>,
{
    pub fn draw<S: Into<ShapeStyle>>(
        &self,
        chart: &mut ChartContext<
            '_,
            BitMapBackend<'_>,
            Cartesian2d<
                plotters::coord::types::RangedCoordf64,
                plotters::coord::types::RangedCoordf64,
            >,
        >,
        style: S,
    ) {
        let arc = self.to_value();
        let nb_points = 1000;
        let angle = arc.angle().value();
        let start_angle = arc.start_angle().value();
        let nb = ((nb_points as f64) * (angle / (2.0 * std::f64::consts::PI))).ceil() as i32;

        let points: Vec<_> = (0..=nb)
            .map(|x| {
                let rad =
                    start_angle + (x as f64) * 2.0 * std::f64::consts::PI / (nb_points as f64);
                (
                    arc.circle().center().x + arc.circle().radius() * rad.cos(),
                    arc.circle().center().y + arc.circle().radius() * rad.sin(),
                )
            })
            .collect();

        chart.draw_series(LineSeries::new(points, style)).unwrap();
    }
}

//-------------------------------------------------- Segment --------------------------------------------------

impl<T> Segment<T>
where
    T: Number,
    T: HasValue<Output = f64>,
{
    pub fn draw<S: Into<ShapeStyle>>(
        &self,
        chart: &mut ChartContext<
            '_,
            BitMapBackend<'_>,
            Cartesian2d<
                plotters::coord::types::RangedCoordf64,
                plotters::coord::types::RangedCoordf64,
            >,
        >,
        style: S,
    ) {
        let segment = self.to_value();
        let point1 = segment.first().into();
        let point2 = segment.second().into();

        chart
            .draw_series(LineSeries::new(vec![point1, point2], style))
            .unwrap();
    }
}

//-------------------------------------------------- Pose --------------------------------------------------

impl<T> Pose<T>
where
    T: Number,
    T: HasValue<Output = f64>,
{
    pub fn draw<S: Into<ShapeStyle>>(
        &self,
        chart: &mut ChartContext<
            '_,
            BitMapBackend<'_>,
            Cartesian2d<
                plotters::coord::types::RangedCoordf64,
                plotters::coord::types::RangedCoordf64,
            >,
        >,
        point_color: RGBColor,
        point_size: i32,
        line_style: S,
    ) {
        let pose = self.to_value();

        pose.position().draw(chart, point_color, point_size);

        let point1 = pose.position().into();
        let point2 = (pose.position() + pose.orientation()).into();

        chart
            .draw_series(LineSeries::new(vec![point1, point2], line_style))
            .unwrap();
    }
}

//-------------------------------------------------- Polygon --------------------------------------------------

impl<T> Polygon<T>
where
    T: Number,
    T: HasValue<Output = f64>,
{
    pub fn draw<S: Into<ShapeStyle> + Clone>(
        &self,
        chart: &mut ChartContext<
            '_,
            BitMapBackend<'_>,
            Cartesian2d<
                plotters::coord::types::RangedCoordf64,
                plotters::coord::types::RangedCoordf64,
            >,
        >,
        style: S,
    ) {
        for seg in self.segments() {
            seg.draw(chart, style.clone());
        }
    }
}