#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;
    use geomety::geometry2d::*;
    use sity::*;

    #[test]
    fn test_position_to_position() {
        let pt1 = Position::new(SI::cm(0.0), SI::cm(3.0));
        let pt2 = Position::new(SI::cm(4.0), SI::cm(0.0));
        let distance = pt1.distance(&pt2);
        assert_approx_eq!(distance, SI::cm(5.0), SI::cm(1e-10));
    }

    #[test]
    fn test_position_to_line_0() {
        let pt = Position::new(SI::cm(1.0), SI::cm(1.0));
        let p = Position::new(SI::cm(1.0), SI::cm(0.0));
        let v = Vector::new(SI::cm(0.0), SI::cm(-1.0));
        let line = Line::new(p, v);
        let distance = pt.distance(&line);
        assert_approx_eq!(distance, SI::cm(0.0), SI::cm(1e-10));
    }

    #[test]
    fn test_position_to_line_1() {
        let pt = Position::new(SI::cm(1.0), SI::cm(1.0));
        let p = Position::new(SI::cm(5.0), SI::cm(0.0));
        let v = Vector::new(SI::cm(0.0), SI::cm(1.0));
        let line = Line::new(p, v);
        let distance = pt.distance(&line);
        assert_approx_eq!(distance, SI::cm(4.0), SI::cm(1e-10));
    }

    #[test]
    fn test_position_to_line_2() {
        let pt = Position::new(SI::cm(1.0), SI::cm(1.0));
        let p = Position::new(SI::cm(0.0), SI::cm(5.0));
        let v = Vector::new(SI::cm(-1.0), SI::cm(0.0));
        let line = Line::new(p, v);
        let distance = pt.distance(&line);
        assert_approx_eq!(distance, SI::cm(4.0), SI::cm(1e-10));
    }

    #[test]
    fn test_position_to_line_3() {
        let pt = Position::new(SI::cm(1.0), SI::cm(2.0));
        let p = Position::new(SI::cm(1.0), SI::cm(0.0));
        let v = Vector::new(SI::cm(1.0), SI::cm(1.0));
        let line = Line::new(p, v);
        let distance = pt.distance(&line);
        assert_approx_eq!(distance, SI::cm(2.0.root2()), SI::cm(1e-10));
    }
}
