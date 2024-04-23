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
    stop_angle: f64, // Added stop_angle field
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
    fn new_rotational(center: (f64, f64), radius: f64, clockwise: bool, stop_angle: f64) -> Self {
        Motion::Rotational(RotationalMotion {
            center,
            radius,
            clockwise,
            stop_angle, // Added stop_angle initialization
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

        // Check if there are at least 3 parts (to avoid panics)
        if parts.len() < 3 {
            println!("Invalid command format: {}", line);
            continue;
        }

        // Check if the command is "LIN"
        if parts[0] == "LIN" {
            // Parse start and end points from the parts
            let start = (
                parts[1][1..].parse().unwrap_or(0.0), // Parse X coordinate
                parts[2][1..].parse().unwrap_or(0.0), // Parse Y coordinate
                parts[3][1..].parse().unwrap_or(0.0), // Parse Z coordinate
            );
            motions.push(Motion::new_linear(prev_start, start)); // Use previous start point as end point
            prev_start = start; // Update previous start point
        } else if parts[0] == "CW" || parts[0] == "CCW" {
            // Ensure that the CW or CCW command has at least 5 parts
            if parts.len() < 5 {
                println!("Invalid command format: {}", line);
                continue;
            }

            // Parse parameters for rotational motion
            let center = (
                parts[1][1..].parse().unwrap_or(0.0), // Parse X coordinate
                parts[2][1..].parse().unwrap_or(0.0), // Parse Y coordinate
            );
            let radius = parts[3][1..].parse().unwrap_or(0.0); // Parse radius
            let stop_angle = parts[4][1..].parse().unwrap_or(0.0); // Parse stop angle
            // Create a new rotational motion and push it to the vector
            motions.push(Motion::new_rotational(center, radius, parts[0] == "CW", stop_angle));
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
                        println!("Rotational Motion: {:?}", rotational_motion);
                        let positions = rotational_motion_calculate(rotational_motion);
                        for (x, y) in positions {
                            println!("{:.2}, {:.2}", x, y);
                        }
                    }
                }
            }
        }
        Err(e) => println!("Error reading file: {}", e),
    }
}

// Function to calculate positions for linear motion
fn linear_motion_calculate(start: (f64, f64, f64), end: (f64, f64, f64)) -> Vec<(f64, f64, f64)> {
    // Calculate the total change in each dimension
    let dx = end.0 - start.0;
    let dy = end.1 - start.1;
    let dz = end.2 - start.2;

    // Determine the number of steps based on the change in the y dimension
    let num_steps = dy.abs().ceil() as usize;

    // Generate positions for each step
    let mut positions = Vec::new();
    for i in 0..=num_steps {
        let t = i as f64 / num_steps as f64;
        let x = start.0 + dx * t;
        let y = start.1 + dy * t;
        let z = start.2 + dz * t;
        positions.push((x, y, z));
    }

    positions
}



// Function to calculate positions for rotational motion
fn rotational_motion_calculate(rotational_motion: RotationalMotion) -> Vec<(f64, f64)> {
    // Define constants for full circle and degree to radian conversion
    const FULL_CIRCLE: f64 = std::f64::consts::PI * 2.0;
    const DEG_TO_RAD: f64 = std::f64::consts::PI / 180.0;

    // Determine the step angle based on the radius
    let step_angle = 5.0 / rotational_motion.radius;

    // Calculate the start and end angles based on the direction of rotation
    let (start_angle, end_angle) = if rotational_motion.clockwise {
        (0.0, rotational_motion.stop_angle)
    } else {
        (FULL_CIRCLE, FULL_CIRCLE - rotational_motion.stop_angle)
    };

    // Generate positions at 5-degree intervals
    let mut positions = Vec::new();
    let mut angle = start_angle;
    while angle <= end_angle {
        let x = rotational_motion.center.0 + rotational_motion.radius * angle.cos();
        let y = rotational_motion.center.1 + rotational_motion.radius * angle.sin();
        positions.push((x, y));
        angle += DEG_TO_RAD * step_angle;
    }

    positions
}
