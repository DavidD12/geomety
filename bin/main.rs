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

    let v2: Vector<_> = (10.0, 20.0).into();
    let vector = (3.0 * vector) + v2 / 3.0;
    println!("{}", vector.norm());
}
