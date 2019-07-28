use rand::Rng;

struct Point2D {
    pub x: f64,
    pub y: f64,
}

fn is_inside_unit_circle(point: &Point2D) -> bool {
    (point.x * point.x + point.y * point.y).sqrt() < 1.0
}

fn approximate_pi(max_point: u64) -> f64 {
    let mut rng = rand::thread_rng();
    let mut points_in_circle = 0;

    for _ in 0..max_point {
        let point = Point2D {
            x: rng.gen_range(0.0, 1.0),
            y: rng.gen_range(0.0, 1.0),
        };

        if is_inside_unit_circle(&point) {
            points_in_circle += 1;
        }
    }
    println!(
        "{} points in circle out of {} total points.",
        points_in_circle, max_point
    );

    4.0f64 * f64::from(points_in_circle) / max_point as f64
}

fn main() {
    println!("{}", approximate_pi(10000));
}
