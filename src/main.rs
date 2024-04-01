use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// Define a struct to represent linear motion
struct LinearMotion {
    start: (f64, f64, f64),
    end: (f64, f64, f64),
}

// Define a struct to represent rotational motion
#[derive(Debug)] 
struct RotationalMotion {
    center: (f64, f64),
    radius: f64,
    clockwise: bool,
    stop_angle: f64,
    i: Option<f64>,
    j: Option<f64>,
    k: Option<f64>,
}

// Define an enum to represent different types of motion
enum Motion {
    Linear(LinearMotion),
    Rotational(RotationalMotion),
}

impl Motion {
    // Constructor for linear motion
    fn new_linear(start: (f64, f64, f64), end: (f64, f64, f64)) -> Self {
        Motion::Linear(LinearMotion { start, end })
    }

    // Constructor for rotational motion
    fn new_rotational(center: (f64, f64), radius: f64, clockwise: bool, stop_angle: f64, i: Option<f64>, j: Option<f64>, k: Option<f64>) -> Self {
        Motion::Rotational(RotationalMotion {
            center,
            radius,
            clockwise,
            stop_angle,
            i,
            j,
            k,
        })
    }
}

// Function to read motions from a file
fn read_file(file_path: &str) -> io::Result<Vec<Motion>> {
    // Open the file
    let file = File::open(file_path)?;
    // Create a buffered reader
    let reader = io::BufReader::new(file);
    // Initialize a vector to store motions
    let mut motions = Vec::new();
    let mut prev_start = (0.0, 0.0, 0.0);

    // Iterate through each line in the file
    for line in reader.lines() {
        // Read the line and handle any potential I/O errors
        let line = line?;
        // Split the line into parts using whitespace as delimiter
        let parts: Vec<&str> = line.trim().split_whitespace().collect();

        println!("Parts: {:?}", parts); // Debug print

        // Check if there are at least 4 parts (to avoid panics)
        if parts.len() < 4 {
            println!("Invalid command format: {}", line);
            continue;
        }

        // Check if the command is "LIN"
        if parts[0] == "LIN" {
            println!("Found LIN command");
            // Parse start and end points from the parts
            let start = (
                parts[1][1..].parse().unwrap_or(0.0), // Parse X coordinate
                parts[2][1..].parse().unwrap_or(0.0), // Parse Y coordinate
                parts[3][1..].parse().unwrap_or(0.0), // Parse Z coordinate
            );
            motions.push(Motion::new_linear(prev_start, start)); // Use previous start point as end point
            prev_start = start; // Update previous start point
        } else if parts[0] == "CW" || parts[0] == "CCW" {
            println!("Found CW/CCW command");
            // Ensure that the CW or CCW command has at least 7 parts
            if parts.len() < 7 {
                println!("Invalid command format: {}", line);
                continue;
            }

            // Parse parameters for rotational motion
            let center = (
                parts[1][1..].parse().unwrap_or(0.0), // Parse X coordinate
                parts[2][1..].parse().unwrap_or(0.0), // Parse Y coordinate
            );
            let radius = parts[3][1..].parse().unwrap_or(0.0); // Parse radius
            let clockwise = parts[0] == "CW"; // Determine direction of rotation
            let stop_angle = parts[6][1..].parse().unwrap_or(0.0); // Parse stop angle

            // Optional I, J, K coordinates
            let i = if parts.len() > 4 { Some(parts[4][1..].parse().unwrap_or(0.0)) } else { None };
            let j = if parts.len() > 5 { Some(parts[5][1..].parse().unwrap_or(0.0)) } else { None };
            let k = if parts.len() > 6 { Some(parts[6][1..].parse().unwrap_or(0.0)) } else { None };
            // Create a new rotational motion and push it to the vector
            motions.push(Motion::new_rotational(center, radius, clockwise, stop_angle, i, j, k));
        } else {
            // Handle unrecognized command
            println!("Invalid command: {}", line);
        }
    }

    // Return the vector of motions
    Ok(motions)
}

fn main() {
    // Command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if the correct number of arguments is provided
    if args.len() != 2 {
        println!("Usage: {} <filename.cmmd>", args[0]);
        return;
    }

    // Extract file path from command-line arguments
    let file_path = &args[1];
    // Extract file extension
    let extension = Path::new(file_path)
        .extension()
        .unwrap_or_default()
        .to_str()
        .unwrap_or_default();

    // Check if the file extension is correct
    if extension != "cmmd" {
        println!("Invalid file extension. The file must have a .cmmd extension.");
        return;
    }

    // Attempt to read motions from the file
    match read_file(file_path) {
        Ok(motions) => {
            // Process each motion
            for motion in motions {
                match motion {
                    // Handle linear motion
                    Motion::Linear(linear_motion) => {
                        println!("LIN {:?} to {:?}", linear_motion.start, linear_motion.end);
                        let positions = linear_motion_calculate(linear_motion.start, linear_motion.end);
                        for (x, y, z) in positions {
                            println!("{:.2}, {:.2}, {:.2}", x, y, z);
                        }
                    }
                    // Handle rotational motion
                    Motion::Rotational(rotational_motion) => {
                        println!("Rotational Motion: {:?}", rotational_motion); // Debug print
                        // Calculate positions along the circular path
                        let positions = rotational_motion_calculate(
                            rotational_motion.center,
                            rotational_motion.radius,
                            rotational_motion.clockwise,
                            rotational_motion.stop_angle,
                            rotational_motion.i,  // Pass I coordinate
                            rotational_motion.j,  // Pass J coordinate
                            rotational_motion.k,  // Pass K coordinate
                        );
                        // Print each position
                        for (x, y) in positions {
                            println!("{:.2}, {:.2}", x, y);
                        }
                    }
                }
            }
        }
        // Handle file read error
        Err(e) => println!("Error reading file: {}", e),
    }
}

// Function to calculate positions along a linear path
fn linear_motion_calculate(start: (f64, f64, f64), end: (f64, f64, f64)) -> Vec<(f64, f64, f64)> {
    let mut positions = Vec::new();
    let (start_x, start_y, start_z) = start;
    let (end_x, end_y, end_z) = end;

    println!("Start: {:?}, End: {:?}", start, end); // Debug print

    // Calculate intermediate positions along the linear path
    for i in 0..=100 {
        let ratio = i as f64 / 100.0;
        let x = start_x + ratio * (end_x - start_x);
        let y = start_y + ratio * (end_y - start_y);
        let z = start_z + ratio * (end_z - start_z);
        positions.push((x, y, z));
    }

    positions
}

// Function to calculate positions along a rotational path
fn rotational_motion_calculate(center: (f64, f64), radius: f64, clockwise: bool, stop_angle: f64, i: Option<f64>, j: Option<f64>, k: Option<f64>) -> Vec<(f64, f64)> {
    let mut positions = Vec::new();
    let direction = if clockwise { -1.0 } else { 1.0 };

    // Normalize I, J, K coordinates to get a unit direction vector
    let (i, j, k) = match (i, j, k) {
        (Some(i), Some(j), Some(k)) => {
            let magnitude = (i * i + j * j + k * k).sqrt();
            if magnitude == 0.0 {
                // If the magnitude is zero, choose a default direction along the X-axis
                (1.0, 0.0, 0.0)
            } else {
                (i / magnitude, j / magnitude, k / magnitude)
            }
        }
        _ => (1.0, 0.0, 0.0), // Default direction along the X-axis
    };

    // Calculate positions along the circular path at 5-degree intervals
     for angle in (0..=stop_angle as i32).step_by(5) {
        let radians = (angle as f64).to_radians();
        let x = center.0 + radius * radians.cos();
        let y = center.1 + radius * radians.sin();
        println!("Angle: {}, X: {:.2}, Y: {:.2}", angle, x, y); // Debug print
        positions.push((x, y));
    }

    positions
}
