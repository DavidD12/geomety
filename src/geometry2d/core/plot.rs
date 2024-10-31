use super::{Circle as GCircle, Polygon, Segment};

use plotters::prelude::*;

pub trait Plot {
    fn draw(
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
    );
}

impl Plot for GCircle<f64> {
    fn draw(
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
    ) {
        let points: Vec<_> = (0..=3600)
            .map(|deg| {
                let rad = (deg as f64) / 10.0 * std::f64::consts::PI / 180.0;
                (
                    self.center().x + self.radius() * rad.cos(),
                    self.center().y + self.radius() * rad.sin(),
                )
            })
            .collect();

        chart.draw_series(LineSeries::new(points, color)).unwrap();
    }
}

impl Plot for Segment<f64> {
    fn draw(
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
    ) {
        let point1 = (self.first().x, self.first().y);
        let point2 = (self.second().x, self.second().y);

        chart
            .draw_series(LineSeries::new(vec![point1, point2], color))
            .unwrap();
    }
}

impl Plot for Polygon<f64> {
    fn draw(
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
    ) {
        // let points: Vec<_> = self.points().iter().map(|p| (p.x, p.y)).collect();
        // chart.draw_series(LineSeries::new(points, style)).unwrap();
        for seg in self.segments() {
            seg.draw(chart, color.clone());
        }
    }
}
