use clap::Parser;
use std::f64::consts::PI;

fn num_in_circle(row: f64) -> i64 {
    (2.0 * PI * row).round() as i64
}

fn num_in_sphere(sphere_radius: f64, row: f64) -> i64 {
    let rad = row / sphere_radius;
    let radius = (sphere_radius * rad.sin()).abs();
    (2.0 * PI * radius).round() as i64
}

fn make_string(a: i64, b: i64) -> String {
    let ratio = b as f64 / b as f64 - a as f64;
    if a == 0 || ratio >= 2.0 {
        "V".repeat((b / 2) as usize)
    } else {
        let mut ones_and_twos = Vec::new();
        for _ in 0..a {
            let factor = ones_and_twos.len() as f64 / a as f64;
            if (factor * b as f64) < ones_and_twos.iter().sum::<i32>() as f64 {
                ones_and_twos.push(1);
            } else {
                ones_and_twos.push(2);
            }
        }
        ones_and_twos
            .iter()
            .map(|&x| if x == 1 { "|" } else { "V" })
            .collect::<String>()
    }
}

#[derive(Debug, Clone)]
enum CrochetType {
    Circle,
    Sphere { radius: f64 },
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(value_name = "TYPE", help = "crochet type ('circle' or 'sphere')")]
    crochet_type: String,

    #[arg(value_name = "ROW", help = "starting row number", default_value_t = 0)]
    row: i64,

    #[arg(short, long, value_name = "RADIUS", help = "sphere radius (for 'sphere' type)")]
    sphere_radius: Option<f64>,
}


fn main() {
    let args = Args::parse();
    const NUM_ROWS: i64 = 10;

    let crochet_type = match args.crochet_type.as_str() {
        "circle" => CrochetType::Circle,
        "sphere" => {
            if let Some(radius) = args.sphere_radius {
                CrochetType::Sphere { radius }
            } else {
                eprintln!("Error: sphere radius is required for 'sphere' type");
                std::process::exit(1);
            }
        }
        _ => {
            eprintln!("Error: invalid crochet type (must be 'circle' or 'sphere')");
            std::process::exit(1);
        }
    };


    if let CrochetType::Sphere { radius } = &crochet_type {
        println!("sphere radius: {}", radius);
    }

    let rows: Vec<i64> = match &crochet_type {
        CrochetType::Circle => (args.row..args.row + NUM_ROWS).collect(),
        CrochetType::Sphere { radius } => (0..((PI * radius).round() as i64)).collect(),
    };

    let rad: Vec<i64> = match &crochet_type {
        CrochetType::Circle => rows.iter().map(|&row| num_in_circle(row as f64)).collect(),
        CrochetType::Sphere { radius } => rows.iter().map(|&row| num_in_sphere(*radius, row as f64)).collect(),
    };

    let pairs: Vec<(&i64, &i64)> = rad.iter().zip(rad.iter().skip(1)).collect();

    for (idx, (a, b)) in pairs.iter().enumerate() {
        println!(
            "{:2}: {:3} {:3} -> {}",
            args.row + idx as i64,
            a,
            b,
            make_string(**a, **b)
        );
    }
}
