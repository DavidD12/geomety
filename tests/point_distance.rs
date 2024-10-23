#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;
    use geomety::geometry2d::*;
    use sity::*;

    #[test]
    fn test_point_to_point() {
        let pt1 = Point::new(0.0, 3.0);
        let pt2 = Point::new(4.0, 0.0);
        let distance = pt1.distance(&pt2);
        assert_approx_eq!(distance, 5.0, 1e-10);
    }

    #[test]
    fn test_point_to_line_0() {
        let pt = Point::new(1.0, 1.0);
        let p = Point::new(1.0, 0.0);
        let v = Vector::new(0.0, -1.0);
        let line = Line::new(p, v);
        let distance = pt.distance(&line);
        assert_approx_eq!(distance, 0.0, 1e-10);
    }

    #[test]
    fn test_point_to_line_1() {
        let pt = Point::new(1.0, 1.0);
        let p = Point::new(5.0, 0.0);
        let v = Vector::new(0.0, 1.0);
        let line = Line::new(p, v);
        let distance = pt.distance(&line);
        assert_approx_eq!(distance, 4.0, 1e-10);
    }

    #[test]
    fn test_point_to_line_2() {
        let pt = Point::new(1.0, 1.0);
        let p = Point::new(0.0, 5.0);
        let v = Vector::new(-1.0, 0.0);
        let line = Line::new(p, v);
        let distance = pt.distance(&line);
        assert_approx_eq!(distance, 4.0, 1e-10);
    }

    #[test]
    fn test_point_to_line_3() {
        let pt = Point::new(1.0, 2.0);
        let p = Point::new(1.0, 0.0);
        let v = Vector::new(1.0, 1.0);
        let line = Line::new(p, v);
        let distance = pt.distance(&line);
        assert_approx_eq!(distance, 2.0.root2(), 1e-10);
    }

    // #[test]
    // fn test_point_to_segment_0() {
    //     let pt = Point::new(2.0, 0.0);
    //     let p1 = Point::new(2.0, -1.0);
    //     let p2 = Point::new(2.0, 1.0);
    //     let seg = Segment::new(p1, p2);
    //     let distance = pt.distance(&seg);
    //     assert_approx_eq!(distance, 0.0, 1e-10);
    // }
    // #[test]
    // fn test_point_to_segment_1() {
    //     let pt = Point::new(2.0, 5.0);
    //     let p1 = Point::new(2.0, -1.0);
    //     let p2 = Point::new(2.0, 1.0);
    //     let seg = Segment::new(p1, p2);
    //     let distance = pt.distance(&seg);
    //     assert_approx_eq!(distance, 4.0, 1e-10);
    // }

    // #[test]
    // fn test_point_to_segment_2() {
    //     let pt = Point::new(2.0, -5.0);
    //     let p1 = Point::new(2.0, -1.0);
    //     let p2 = Point::new(2.0, 1.0);
    //     let seg = Segment::new(p1, p2);
    //     let distance = pt.distance(&seg);
    //     assert_approx_eq!(distance, 4.0, 1e-10);
    // }

    // #[test]
    // fn test_point_to_segment_3() {
    //     let pt = Point::new(0.0, 0.0);
    //     let p1 = Point::new(2.0, -1.0);
    //     let p2 = Point::new(2.0, 1.0);
    //     let seg = Segment::new(p1, p2);
    //     let distance = pt.distance(&seg);
    //     assert_approx_eq!(distance, 2.0, 1e-10);
    // }
}