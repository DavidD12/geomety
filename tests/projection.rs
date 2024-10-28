#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;
    use geomety::geometry2d::*;

    #[test]
    fn projection_line_0() {
        let p = Point::new(1.0, 1.0);
        let v = Vector::new(0.0, 1.0);
        let l = Line::new(p, v);
        let pt = Point::new(1.0, 0.0);
        let pt = pt.projection_to_line(&l);
        assert_approx_eq!(pt.x, 1.0_f64, 1e-10);
        assert_approx_eq!(pt.y, 0.0_f64, 1e-10);
    }

    #[test]
    fn projection_line_1() {
        let p = Point::new(1.0, 1.0);
        let v = Vector::new(0.0, 1.0);
        let l = Line::new(p, v);
        let pt = Point::new(0.0, 0.0);
        let pt = pt.projection_to_line(&l);
        assert_approx_eq!(pt.x, 1.0_f64, 1e-10);
        assert_approx_eq!(pt.y, 0.0_f64, 1e-10);
    }

    #[test]
    fn projection_segment_0() {
        let p1 = Point::new(1.0, -1.0);
        let p2 = Point::new(1.0, 1.0);
        let s = Segment::new(p1, p2);
        let pt = Point::new(1.0, 0.0);
        let opt = pt.projection_to_segment(&s);
        assert_eq!(opt.is_some(), true);
        let pt = opt.unwrap();
        assert_approx_eq!(pt.x, 1.0_f64, 1e-10);
        assert_approx_eq!(pt.y, 0.0_f64, 1e-10);
    }

    #[test]
    fn projection_segment_1() {
        let p1 = Point::new(1.0, -1.0);
        let p2 = Point::new(1.0, 1.0);
        let s = Segment::new(p1, p2);
        let pt = Point::new(0.0, 0.0);
        let opt = pt.projection_to_segment(&s);
        assert_eq!(opt.is_some(), true);
        let pt = opt.unwrap();
        assert_approx_eq!(pt.x, 1.0_f64, 1e-10);
        assert_approx_eq!(pt.y, 0.0_f64, 1e-10);
    }

    #[test]
    fn projection_segment_2() {
        let p1 = Point::new(1.0, -1.0);
        let p2 = Point::new(1.0, 1.0);
        let s = Segment::new(p1, p2);
        let pt = Point::new(0.0, 2.0);
        let opt = pt.projection_to_segment(&s);
        assert_eq!(opt.is_some(), false);
    }

    #[test]
    fn projection_segment_3() {
        let p1 = Point::new(1.0, -1.0);
        let p2 = Point::new(1.0, 1.0);
        let s = Segment::new(p1, p2);
        let pt = Point::new(0.0, -2.0);
        let opt = pt.projection_to_segment(&s);
        assert_eq!(opt.is_some(), false);
    }
}
