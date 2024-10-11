use geometry2d::*;
use geomety::*;
use sity::*;

fn main() {
    let x = SI::cm(2.0);
    let y = SI::cm(3.0);

    let vector = Vector::new(2.0, 3.0);
    let point = Point::new(x, y);

    println!("{}", vector);
    println!("{}", point);

    println!("--------------------------------------------------");
    let v1: Vector<_> = (10.0, 20.0).into();
    let v2 = (3.0 * vector) + v1 / 3.0;
    println!("{}", v1);
    println!("{}", v1.length());

    println!("--------------------------------------------------");
    let v: Vector<_> = (x, y).into();
    println!("{}", v);
    let v = v / v;

    println!("--------------------------------------------------");
    let v: Vector<_> = (SI::m(10.0), SI::m(20.0)).into();
    println!("{}", v);
    println!("{}", v.length());

    let v: Vector<_> = (10.0, 20.0).into();
    println!("{}", v);
    println!("{}", v.normalize());
    //
    let v: Vector<_> = (SI::m(10.0), SI::m(20.0)).into();
    println!("{}", v);
    println!("{}", v.normalize());
}
