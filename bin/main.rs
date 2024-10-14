use geometry2d::*;
use geomety::*;
use sity::*;

fn main() {
    let x = SI::cm(2.0);
    let y = SI::cm(3.0);
    let v1: Vector<_> = (x, y).into();
    let v2: Vector<_> = (2.0, 3.0).into();

    let p1: Point<f64, Centi> = Point::default();
    let l1 = Line::new(p1, v1);

    println!("{}", v1);
    println!("{}", v2);

    println!("{}", is_parallel(&v1, &v1));
    println!("{}", is_parallel(&v2, &v2));

    println!("{}", is_parallel(&l1, &l1));

    println!("{}", v1.is_parallel(&v1));
    println!("{}", v2.is_parallel(&v2));
    println!("{}", v1.is_parallel(&l1));

    println!("{}", l1.is_parallel(&l1));
    println!("{}", l1.is_parallel(&v1));

    // let vector = Vector::new(2.0, 3.0);
    // let point = Point::new(x, y);

    // println!("{}", vector);
    // println!("{}", point);

    // println!("--------------------------------------------------");
    // let v1: Vector<_> = (10.0, 20.0).into();
    // let v2 = (3.0 * vector) + v1 / 3.0;
    // println!("{}", v1);
    // println!("{}", v1.length());

    // println!("--------------------------------------------------");
    // let v: Vector<_> = (x, y).into();
    // println!("{}", v);
    // let v = v / v;

    // println!("--------------------------------------------------");
    // let v: Vector<_> = (SI::m(10.0), SI::m(20.0)).into();
    // println!("{}", v);
    // println!("{}", v.length());

    // let v: Vector<_> = (10.0, 20.0).into();
    // println!("{}", v);
    // println!("{}", v.normalize());
    // //
    // let v: Vector<_> = (SI::m(10.0), SI::m(20.0)).into();
    // println!("{}", v);
    // println!("{}", v.normalize());

    // println!("--------------------------------------------------");
}
