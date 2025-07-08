use geomety::geometry2d::*;
use sity::*;

fn main() {
    plot_path();
}

fn plot_path() {
    let p1 = Point::new(metre(1.0), metre(0.0));
    let p2 = Point::new(metre(4.0), metre(2.0));
    let p3 = Point::new(metre(3.0), metre(4.0));
    let p4 = Point::new(metre(1.0), metre(3.0));
    let p5 = Point::new(metre(0.0), metre(1.0));
    let points = vec![p1, p2, p3, p4, p5];
    let p = Polygon::convex_hull(points).unwrap();
    //
    let radius = metre(0.15);
    let distance = metre(0.5);
    // let map_direction = Vector::new(metre(1.0), metre(0.0));
    let start = Pose::new(
        Point::new(metre(4.0), metre(0.0)),
        Vector::new(metre(0.0), metre(0.5)),
    );

    // let segments = p.mapping(&map_direction, distance);
    // let point = p.mapping_first_point(&map_direction);
    // let path = Path::mapping(&start, &map_direction, radius, distance, &p).unwrap();
    let path = Path::optimal_mapping(&start, radius, distance, &p).unwrap();

    println!("length = {}", path.length());
    let traj = path.trajectories().get(0).unwrap();

    let velocity = metre(1.0) / second(1.0);
    let duration = second(2.5);
    let res = path.follow(velocity, duration);

    let traj_res = traj.segment().follow(velocity, duration);
    println!(
        "{} {:?}",
        traj_res.pose,
        traj_res.complete.map(|x| x.value())
    );

    {
        use plotters::prelude::*;
        let root = BitMapBackend::new("geometrie.png", (1000, 1000)).into_drawing_area();
        root.fill(&WHITE).unwrap();

        let x_min = -1.0;
        let x_max = 5.0;
        let y_min = -1.0;
        let y_max = 5.0;

        let mut chart: ChartContext<
            '_,
            BitMapBackend<'_>,
            Cartesian2d<
                plotters::coord::types::RangedCoordf64,
                plotters::coord::types::RangedCoordf64,
            >,
        > = ChartBuilder::on(&root)
            // .caption("y=x^2", ("sans-serif", 50).into_font())
            .margin(5)
            .x_label_area_size(50)
            .y_label_area_size(50)
            .build_cartesian_2d(x_min..x_max, y_min..y_max)
            .unwrap();
        chart.configure_mesh().draw().unwrap();
        //
        p.draw(&mut chart, BLACK);
        start.draw(&mut chart, RED, 5, RED.stroke_width(2), 1.0);
        // for seg in segments.iter() {
        //     seg.draw(&mut chart, RGBAColor(50, 50, 50, 0.5).stroke_width(10));
        // }
        path.draw(&mut chart, BLUE.stroke_width(2));
        // point.draw(&mut chart, BLACK, 5);
        res.pose.draw(&mut chart, BLUE, 5, BLUE, 1.0);
        //
        root.present().unwrap();
    }
}

pub fn plot_trajectory() {
    let p = Point::new(metre(2.0), metre(7.0));
    let v = Vector::new(metre(1.0), metre(1.0));
    let start = Pose::new(p, v);
    let p = Point::new(metre(7.0), metre(3.0));
    let v = Vector::new(metre(-1.0), metre(0.0));
    let finish = Pose::new(p, v);
    let radius = metre(1.0);

    let list = Trajectory::create_all(&start, &finish, radius);
    let trajectory = Trajectory::create(&start, &finish, radius).unwrap();

    {
        use plotters::prelude::*;
        let root = BitMapBackend::new("geometrie.png", (1000, 1000)).into_drawing_area();
        root.fill(&WHITE).unwrap();

        let x_min = 0.0;
        let x_max = 10.0;
        let y_min = 0.0;
        let y_max = 10.0;

        let mut chart: ChartContext<
            // '_,
            BitMapBackend<'_>,
            Cartesian2d<
                plotters::coord::types::RangedCoordf64,
                plotters::coord::types::RangedCoordf64,
            >,
        > = ChartBuilder::on(&root)
            // .caption("y=x^2", ("sans-serif", 50).into_font())
            .margin(5)
            .x_label_area_size(50)
            .y_label_area_size(50)
            .build_cartesian_2d(x_min..x_max, y_min..y_max)
            .unwrap();
        chart.configure_mesh().draw().unwrap();
        //
        start.draw(&mut chart, GREEN, 5, GREEN.stroke_width(2), 1.0);
        finish.draw(&mut chart, RED, 5, RED.stroke_width(2), 1.0);

        for traj in list.iter() {
            traj.draw(&mut chart, RGBAColor(200, 200, 200, 1.0).stroke_width(2));
        }
        trajectory.draw(&mut chart, BLUE.stroke_width(2));
        //
        root.present().unwrap();
    }
}

pub fn plot_test() {
    let radius = metre(0.5);
    let p1 = Point::new(metre(2.0), metre(1.0));
    let c1 = Circle::new(p1, radius);
    let d1 = DirectedCircle::new(c1, Direction::CounterClockWise);
    let p2 = Point::new(metre(4.0), metre(2.0));
    let c2 = Circle::new(p2.clone(), radius);
    let seg = d1.tangents_to_circle(&p2, Direction::ClockWise).unwrap();

    let p = Point::new(metre(0.0), metre(3.0));
    let c = Circle::new(p, metre(0.75));
    let dc = DirectedCircle::new(c, Direction::CounterClockWise);
    let start_angle = Radian::new(0.0);
    let delta_angle = Degree::new(60.0).to_radians();
    let arc = DirectedArc::new(dc, start_angle, delta_angle);

    let p = Point::new(metre(0.0), metre(3.0));
    let v = Vector::new(metre(0.5), metre(0.5));
    let pose = Pose::new(p, v);

    {
        use plotters::prelude::*;
        let root = BitMapBackend::new("geometrie.png", (1000, 1000)).into_drawing_area();
        root.fill(&WHITE).unwrap();

        let x_min = -1.0;
        let x_max = 5.0;
        let y_min = -1.0;
        let y_max = 5.0;

        let mut chart: ChartContext<
            '_,
            BitMapBackend<'_>,
            Cartesian2d<
                plotters::coord::types::RangedCoordf64,
                plotters::coord::types::RangedCoordf64,
            >,
        > = ChartBuilder::on(&root)
            // .caption("y=x^2", ("sans-serif", 50).into_font())
            .margin(5)
            .x_label_area_size(50)
            .y_label_area_size(50)
            .build_cartesian_2d(x_min..x_max, y_min..y_max)
            .unwrap();
        chart.configure_mesh().draw().unwrap();
        //
        d1.circle().draw(&mut chart, BLACK);
        c2.draw(&mut chart, BLACK);
        seg.draw(&mut chart, BLUE.stroke_width(2));
        arc.draw(&mut chart, RED.stroke_width(2));
        arc.start_point().draw(&mut chart, BLACK, 5);
        arc.finish_point().draw(&mut chart, GREEN, 5);
        pose.draw(&mut chart, GREEN, 5, GREEN.stroke_width(2), 5.0);
        //
        root.present().unwrap();
    }
}

pub fn plot_poly() {
    let p1 = Point::new(metre(1.0), metre(0.0));
    let p2 = Point::new(metre(4.0), metre(2.0));
    let p3 = Point::new(metre(3.0), metre(4.0));
    let p4 = Point::new(metre(1.0), metre(3.0));
    let p5 = Point::new(metre(0.0), metre(1.0));
    let points = vec![p1, p2, p3, p4, p5];
    let p = Polygon::convex_hull(points).unwrap();
    let center = p.center();
    //
    let direction = Vector::new(metre(1.0), metre(0.0));
    let segments = p.mapping(&direction, metre(0.25));

    {
        use plotters::prelude::*;
        let root = BitMapBackend::new("geometrie.png", (1000, 1000)).into_drawing_area();
        root.fill(&WHITE).unwrap();

        let x_min = -1.0;
        let x_max = 5.0;
        let y_min = -1.0;
        let y_max = 5.0;

        let mut chart: ChartContext<
            '_,
            BitMapBackend<'_>,
            Cartesian2d<
                plotters::coord::types::RangedCoordf64,
                plotters::coord::types::RangedCoordf64,
            >,
        > = ChartBuilder::on(&root)
            // .caption("y=x^2", ("sans-serif", 50).into_font())
            .margin(5)
            .x_label_area_size(50)
            .y_label_area_size(50)
            .build_cartesian_2d(x_min..x_max, y_min..y_max)
            .unwrap();
        chart.configure_mesh().draw().unwrap();
        //
        p.draw(&mut chart, BLACK);
        center.draw(&mut chart, RED, 5);
        for seg in segments.iter() {
            seg.draw(&mut chart, BLUE);
        }
        //
        root.present().unwrap();
    }
}

pub fn plot_circle_tangents() -> Result<(), Box<dyn std::error::Error>> {
    use plotters::prelude::*;

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
