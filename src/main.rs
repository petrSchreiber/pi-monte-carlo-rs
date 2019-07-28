use rand::Rng;
use std::env;
use std::process::exit;

struct Point2D {
    pub x: f64,
    pub y: f64,
}

fn belongs_unit_circle(point: &Point2D) -> bool {
    (point.x * point.x + point.y * point.y).sqrt() <= 1.0
}

fn approximate_pi(max_point: u64) -> f64 {
    let mut rng = rand::thread_rng();
    let mut points_in_circle = 0;

    for _ in 0..max_point {
        let point = Point2D {
            x: rng.gen_range(0.0, 1.0),
            y: rng.gen_range(0.0, 1.0),
        };

        if belongs_unit_circle(&point) {
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
    let args: Vec<String> = env::args().collect();

    let max_points = if args.len() == 2 {
        match args[1].parse::<u64>() {
            Ok(v) => v,
            Err(_v) => {
                println!("Could not parse passed argument as u64");
                exit(1)
            }
        }
    } else {
        let default_points = 1000;
        println!("Using default amount of points: {}", default_points);

        default_points
    };

    println!("{}", approximate_pi(max_points));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn belongs_unit_circle_for_inside_point() {
        let point_inside = Point2D { x: 0.0, y: 0.0 };

        assert!(belongs_unit_circle(&point_inside));
    }

    #[test]
    fn belongs_unit_circle_for_outside_point() {
        let point_outside = Point2D { x: 2.0, y: 0.0 };

        assert!(!belongs_unit_circle(&point_outside));
    }

    #[test]
    fn belongs_unit_circle_for_edge_point() {
        let point_on_the_edge = Point2D { x: 1.0, y: 0.0 };

        assert!(belongs_unit_circle(&point_on_the_edge));
    }

}
