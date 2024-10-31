#[cfg(test)]
mod tests {
    use geomety::geometry2d::*;

    #[test]
    fn test_convex_hull() {
        let points = vec![
            Point::new(0.0, 3.0),
            Point::new(2.0, 2.0),
            Point::new(1.0, 1.0),
            Point::new(2.0, 1.0),
            Point::new(3.0, 0.0),
            Point::new(0.0, 0.0),
            Point::new(3.0, 3.0),
        ];

        let expected_hull = vec![
            Point::new(0.0, 0.0),
            Point::new(0.0, 3.0),
            Point::new(3.0, 3.0),
            Point::new(3.0, 0.0),
        ];

        let opt = Polygon::convex_hull(points);
        assert_eq!(opt.is_some(), true);
        let hull = opt.unwrap();
        assert_eq!(hull.points(), &expected_hull);
    }
}
