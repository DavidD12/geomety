use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Création d'une zone de dessin
    let root = BitMapBackend::new("geometrie.png", (1000, 1000)).into_drawing_area();
    root.fill(&WHITE)?;

    let x_min = -10.0;
    let x_max = 20.0;
    let y_min = -10.0;
    let y_max = 20.0;

    let mut chart: ChartContext<
        '_,
        BitMapBackend<'_>,
        Cartesian2d<plotters::coord::types::RangedCoordf64, plotters::coord::types::RangedCoordf64>,
    > = ChartBuilder::on(&root)
        // .caption("y=x^2", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(50)
        .y_label_area_size(50)
        .build_cartesian_2d(x_min..x_max, y_min..y_max)?;
    chart.configure_mesh().draw()?;

    {
        use geometry2d::*;
        use geomety::*;

        let p1 = Point::new(0.0, 0.0);
        let r1 = 2.0;
        let c1 = Circle::new(p1, r1);
        let p2 = Point::new(10.0, 2.0);
        let c2 = Circle::new(p2.clone(), r1);

        c1.draw(&mut chart, BLACK);
        c2.draw(&mut chart, BLACK);

        let (s1, s2) = c1.external_tangents_circle(&p2);
        println!("X1 = ({}, {})", s1.first().x, s1.first().y);
        println!("Y1 = ({}, {})", s1.second().x, s1.second().y);
        println!("X2 = ({}, {})", s2.first().x, s2.first().y);
        println!("Y2 = ({}, {})", s2.second().x, s2.second().y);
        s1.draw(&mut chart, BLUE);
        s2.draw(&mut chart, BLUE);
        println!("----------");
        let (s1, s2) = c1.internal_tangents_circle(&p2).unwrap();
        println!("A1 = ({}, {})", s1.first().x, s1.first().y);
        println!("B1 = ({}, {})", s1.second().x, s1.second().y);
        println!("A2 = ({}, {})", s2.first().x, s2.first().y);
        println!("B2 = ({}, {})", s2.second().x, s2.second().y);
        s1.draw(&mut chart, RED);
        s2.draw(&mut chart, RED);

        let points = vec![
            Point::new(0.0, 3.0),
            Point::new(2.0, 2.0),
            Point::new(1.0, 1.0),
            Point::new(2.0, 1.0),
            Point::new(3.0, 0.0),
            Point::new(0.0, 0.0),
            Point::new(3.0, 3.0),
        ];

        let polygon = Polygon::convex_hull(points).unwrap();
        polygon.draw(&mut chart, GREEN);
    }

    root.present()?;
    Ok(())
}

fn old() {
    use geometry2d::*;
    use geomety::*;
    use sity::*;

    let p1 = Point::new(0.0, 0.0);
    let r1 = 2.0;
    let c1 = Circle::new(p1, r1);
    let p2 = Point::new(10.0, 2.0);
    let r2 = 2.0;
    // let c2 = Circle::new(p2, r2);

    let (s1, s2) = c1.external_tangents_circle(&p2);
    println!("X1 = ({}, {})", s1.first().x, s1.first().y);
    println!("Y1 = ({}, {})", s1.second().x, s1.second().y);
    println!("X2 = ({}, {})", s2.first().x, s2.first().y);
    println!("Y2 = ({}, {})", s2.second().x, s2.second().y);
    println!("----------");
    let (s1, s2) = c1.internal_tangents_circle(&p2).unwrap();
    println!("A1 = ({}, {})", s1.first().x, s1.first().y);
    println!("B1 = ({}, {})", s1.second().x, s1.second().y);
    println!("A2 = ({}, {})", s2.first().x, s2.first().y);
    println!("B2 = ({}, {})", s2.second().x, s2.second().y);

    // assert_eq!(t1.is_some(), true);
    // assert_eq!(t2.is_some(), true);

    // let (s1, s2) = t1.unwrap();
    // println!("{}", s1);
    // println!("{}", s2);

    // let p1 = Point::new(1.0, 3.0);
    // let p2 = Point::new(1.0, 1.0);
    // // let p1 = Position::new(metre(1.0), metre(3.0));
    // // let p2 = Position::new(metre(1.0), metre(1.0));
    // let a = p1.angle(&p2);
    // println!("{}", a);

    // let p1 = Point::new(1.0, 2.0);
    // let v1 = Vector::new(3.0, 4.0);
    // let l1 = Line::new(p1, v1);
    // let p2 = Point::new(2.0, 3.0);
    // let v2 = Vector::new(-1.0, 2.0);
    // let l2 = Line::new(p2, v2);

    // match l1.intersection(&l2) {
    //     Some(pt) => println!("{}", pt),
    //     None => println!("no intersection"),
    // };

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
