use geometry2d::*;
use geomety::*;

fn main() {
    let p1 = Point::new(1.0, 2.0);
    let v1 = Vector::new(3.0, 4.0);
    let l1 = Line::new(p1, v1);
    let p2 = Point::new(2.0, 3.0);
    let v2 = Vector::new(-1.0, 2.0);
    let l2 = Line::new(p2, v2);

    match l1.intersection(&l2) {
        Some(pt) => println!("{}", pt),
        None => println!("no intersection"),
    };

    // println!("--------------------------------------------------");
    // let p1 = Point::new(SI::m(1.0), SI::m(2.0));
    // let v1 = Vector::new(SI::m(3.0), SI::m(4.0));
    // let l1 = Line::new(p1, v1);
    // let p2 = Point::new(SI::m(2.0), SI::m(3.0));
    // let v2 = Vector::new(SI::m(-1.0), SI::m(2.0));
    // let l2 = Line::new(p2, v2);

    // match l1.intersection(&l2) {
    //     Some(pt) => println!("{}", pt),
    //     None => println!("no intersection"),
    // };

    // println!("--------------------------------------------------");

    // let v1: Vector<_> = (2.0, 3.0).into();
    // let v2: Vector<_> = (4.0, 5.0).into();
    // println!("v1 = {}", v1);
    // println!("v2 = {}", v2);
    // println!("v1 . v2 = {}", v1.dot_product(&v2));
    // println!("v1 x v2 = {}", v1.cross_product(&v2));

    // println!("--------------------------------------------------");

    // let x1 = SI::cm(2.0);
    // let y1 = SI::cm(3.0);
    // let v1: Vector<_> = (x1, y1).into();
    // let x2 = SI::cm(4.0);
    // let y2 = SI::cm(5.0);
    // let v2: Vector<_> = (x2, y2).into();
    // println!("v1 = {}", v1);
    // println!("v2 = {}", v2);
    // println!("v1 . v2 = {}", v1.dot_product(&v2));
    // println!("v1 x v2 = {}", v1.cross_product(&v2));

    // println!("--------------------------------------------------");
    // let p1: Position<_, _> = (SI::cm(1.0), SI::cm(2.0)).into();
    // println!("p1 = {}", p1);
    // let dx = SI::cm(3.0);
    // let dy = SI::cm(4.0);
    // let dt = SI::ms(12.0);
    // let d: Vector<_> = (dx, dy).into();
    // let v = d / dt;
    // println!("v = {}", v);
    // let p2 = p1 + v * 3.0 * dt;
    // println!("p2 = {}", p2);

    // println!("--------------------------------------------------");

    // let x = SI::cm(2.0);
    // let y = SI::cm(3.0);
    // let v1: Vector<_> = (x, y).into();
    // let v2: Vector<_> = (4.0, 6.0).into();
    // let v3: Vector<_> = (2.0 * x, y * 2.0).into();

    // let p1: Point<f64, Centi> = Point::default();
    // let l1 = Line::new(p1, v1);

    // println!("{}", v1);
    // println!("{}", v2);

    // println!("{}", is_parallel(&v1, &v1));
    // println!("{}", is_parallel(&v2, &v2));
    // println!("{}", is_parallel(&v1, &v3));

    // println!("{}", is_parallel(&l1, &l1));

    // println!("{}", v1.is_parallel(&v1));
    // println!("{}", v2.is_parallel(&v2));
    // println!("{}", v1.is_parallel(&l1));

    // println!("{}", l1.is_parallel(&l1));
    // println!("{}", l1.is_parallel(&v1));

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
