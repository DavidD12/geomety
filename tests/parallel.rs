#[cfg(test)]
mod tests {
    use geomety::geometry2d::*;

    #[test]
    fn test_parallel_vector_0() {
        let v1 = Vector::new(1.0, 3.0);
        let v2 = Vector::new(0.0, 0.0);
        let parallel = v1.is_parallel(&v2);
        assert_eq!(parallel, true);
    }

    #[test]
    fn test_parallel_vector_1() {
        let v1 = Vector::new(1.0, 3.0);
        let v2 = Vector::new(-2.0, -6.0);
        let parallel = v1.is_parallel(&v2);
        assert_eq!(parallel, true);
    }

    #[test]
    fn test_parallel_vector_2() {
        let v1 = Vector::new(1.0, 3.0);
        let v2 = Vector::new(-2.0, -5.0);
        let parallel = v1.is_parallel(&v2);
        assert_eq!(parallel, false);
    }

    #[test]
    fn test_parallel_line_0() {
        let p1 = Point::new(1.0, 1.0);
        let v1 = Vector::new(1.0, 3.0);
        let l1 = Line::new(p1, v1);
        let p2 = Point::new(0.0, 0.0);
        let v2 = Vector::new(0.0, 0.0);
        let l2 = Line::new(p2, v2);
        let parallel = l1.is_parallel(&l2);
        assert_eq!(parallel, true);
    }

    #[test]
    fn test_parallel_line_1() {
        let p1 = Point::new(1.0, 1.0);
        let v1 = Vector::new(1.0, 3.0);
        let l1 = Line::new(p1, v1);
        let p2 = Point::new(0.0, 0.0);
        let v2 = Vector::new(-2.0, -6.0);
        let l2 = Line::new(p2, v2);
        let parallel = l1.is_parallel(&l2);
        assert_eq!(parallel, true);
    }

    #[test]
    fn test_parallel_line_2() {
        let p1 = Point::new(1.0, 1.0);
        let v1 = Vector::new(1.0, 3.0);
        let l1 = Line::new(p1, v1);
        let p2 = Point::new(0.0, 0.0);
        let v2 = Vector::new(-2.0, 6.0);
        let l2 = Line::new(p2, v2);
        let parallel = l1.is_parallel(&l2);
        assert_eq!(parallel, false);
    }

    #[test]
    fn test_parallel_line_segment_0() {
        let p = Point::new(1.0, 1.0);
        let v = Vector::new(1.0, 3.0);
        let l = Line::new(p, v);
        let p1 = Point::new(0.0, 0.0);
        let p2: Point<f64> = Point::new(0.0, 0.0);
        let s = Segment::new(p1, p2);
        let parallel = l.is_parallel(&s);
        assert_eq!(parallel, true);
    }

    #[test]
    fn test_parallel_line_segment_1() {
        let p = Point::new(1.0, 1.0);
        let v = Vector::new(1.0, 3.0);
        let l = Line::new(p, v);
        let p1 = Point::new(0.0, 0.0);
        let p2: Point<f64> = Point::new(-2.0, -6.0);
        let s = Segment::new(p1, p2);
        let parallel = l.is_parallel(&s);
        assert_eq!(parallel, true);
    }

    #[test]
    fn test_parallel_line_segment_2() {
        let p = Point::new(1.0, 1.0);
        let v = Vector::new(1.0, 3.0);
        let l = Line::new(p, v);
        let p1 = Point::new(0.0, 0.0);
        let p2: Point<f64> = Point::new(-2.0, -5.0);
        let s = Segment::new(p1, p2);
        let parallel = l.is_parallel(&s);
        assert_eq!(parallel, false);
    }

    // TODO: Segment/Segment
}
