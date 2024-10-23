#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;
    use geomety::geometry2d::*;

    #[test]
    fn intersection_line_0() {
        let p1 = Point::new(1.0, 1.0);
        let v1 = Vector::new(1.0, 3.0);
        let l1 = Line::new(p1, v1);
        let p2 = Point::new(0.0, 0.0);
        let v2 = Vector::new(-2.0, -6.0);
        let l2 = Line::new(p2, v2);
        let opt = l1.intersection(&l2);
        assert_eq!(opt.is_some(), false);
    }

    #[test]
    fn intersection_line_1() {
        let p1 = Point::new(-5.0, -2.0);
        let v1 = Vector::new(5.0, 2.0);
        let l1 = Line::new(p1, v1);
        let p2 = Point::new(-10.0, 4.0);
        let v2 = Vector::new(10.0, -4.0);
        let l2 = Line::new(p2, v2);
        let opt = l1.intersection(&l2);
        assert_eq!(opt.is_some(), true);
        let pt = opt.unwrap();
        assert_approx_eq!(pt.x, 0.0, 1e-10);
        assert_approx_eq!(pt.y, 0.0, 1e-10);
    }

    #[test]
    fn intersection_line_2() {
        let p1 = Point::new(-5.0, -2.0);
        let v1 = Vector::new(5.0, 2.0);
        let l1 = Line::new(p1, v1);
        let p2 = Point::new(-10.0, 4.0);
        let v2 = Vector::new(10.0, 4.0);
        let l2 = Line::new(p2, v2);
        let opt = l1.intersection(&l2);
        assert_eq!(opt.is_some(), false);
    }

    #[test]
    fn intersection_line_segment_1() {
        let p = Point::new(-5.0, -2.0);
        let v = Vector::new(5.0, 2.0);
        let l = Line::new(p, v);
        let p1 = Point::new(-10.0, 4.0);
        let p2 = Point::new(10.0, -4.0);
        let s = Segment::new(p1, p2);
        let opt = l.intersection(&s);
        assert_eq!(opt.is_some(), true);
        let pt = opt.unwrap();
        assert_approx_eq!(pt.x, 0.0, 1e-10);
        assert_approx_eq!(pt.y, 0.0, 1e-10);
    }

    #[test]
    fn intersection_line_segment_2() {
        let p = Point::new(-5.0, -2.0);
        let v = Vector::new(5.0, 2.0);
        let l = Line::new(p, v);
        let p1 = Point::new(-10.0, 4.0);
        let p2 = Point::new(5.0, 5.0);
        let s = Segment::new(p1, p2);
        let opt = l.intersection(&s);
        assert_eq!(opt.is_some(), false);
    }

    #[test]
    fn intersection_line_segment_3() {
        let p = Point::new(-5.0, -2.0);
        let v = Vector::new(5.0, 2.0);
        let l = Line::new(p, v);
        let p1 = Point::new(-10.0, 4.0);
        let p2 = Point::new(-5.0, 2.0);
        let s = Segment::new(p1, p2);
        let opt = l.intersection(&s);
        assert_eq!(opt.is_some(), false);
    }

    #[test]
    fn intersection_segment_0() {
        let p1 = Point::new(2.0, 4.0);
        let p2 = Point::new(4.0, 4.0);
        let s1 = Segment::new(p1, p2);
        let p1 = Point::new(-2.0, -4.0);
        let p2 = Point::new(-4.0, -4.0);
        let s2 = Segment::new(p1, p2);
        let opt = s1.intersection(&s2);
        assert_eq!(opt.is_some(), false);
    }

    #[test]
    fn intersection_segment_1() {
        let p1 = Point::new(5.0, -2.0);
        let p2 = Point::new(-5.0, 2.0);
        let s1 = Segment::new(p1, p2);
        let p1 = Point::new(-10.0, -4.0);
        let p2 = Point::new(10.0, 4.0);
        let s2 = Segment::new(p1, p2);
        let opt = s1.intersection(&s2);
        assert_eq!(opt.is_some(), true);
        let pt = opt.unwrap();
        assert_approx_eq!(pt.x, 0.0, 1e-10);
        assert_approx_eq!(pt.y, 0.0, 1e-10);
    }
}
