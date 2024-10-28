#[cfg(test)]
mod tests {
    use geomety::geometry2d::*;

    #[test]
    fn test_polygon_contains() {
        let points = vec![
            Point::new(0.0, 0.0),
            Point::new(0.0, 3.0),
            Point::new(3.0, 3.0),
            Point::new(3.0, 0.0),
        ];
        let polygon = Polygon { points };
        let point = Point::new(1.0, 1.0);
        assert_eq!(polygon.contains(&point), true);
        let point = Point::new(-1.0, 1.0);
        assert_eq!(polygon.contains(&point), false);
    }
}
