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
            stop_angle,
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

        // Debug print
        println!("Parts: {:?}", parts);

        // Check if there are at least 3 parts (to avoid panics)
        if parts.len() < 3 {
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
            let clockwise = parts[0] == "CW"; // Determine direction of rotation
            let stop_angle = parts[4][1..].parse().unwrap_or(0.0); // Parse stop angle
            // Create a new rotational motion and push it to the vector
            motions.push(Motion::new_rotational(center, radius, clockwise, stop_angle));
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
                        let positions = rotational_motion_calculate(rotational_motion.center, rotational_motion.radius, rotational_motion.clockwise, rotational_motion.stop_angle);
                        for (x, y) in positions {
                            println!("{:.2}, {:.2}", x, y);
                        }
                    }
                    // Debug print for unmatched motion types
                    _ => println!("Unrecognized motion type"),
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

    // Calculate the maximum distance in any axis
    let max_distance = f64::max(f64::max((end_x - start_x).abs(), (end_y - start_y).abs()), (end_z - start_z).abs());

    // Calculate the number of steps based on the maximum distance
    let steps = max_distance.ceil() as i32;

    // Increment values for each axis
    let x_increment = (end_x - start_x) / steps as f64;
    let y_increment = (end_y - start_y) / steps as f64;
    let z_increment = (end_z - start_z) / steps as f64;

    // Initial position
    let mut x = start_x;
    let mut y = start_y;
    let mut z = start_z;

    // Output the linear motion in one unit increments
    for _ in 0..=steps {
        positions.push((x, y, z));
        x += x_increment;
        y += y_increment;
        z += z_increment;
    }

    positions
}

// Function to calculate positions along a rotational path
fn rotational_motion_calculate(center: (f64, f64), radius: f64, clockwise: bool, stop_angle_degrees: f64) -> Vec<(f64, f64)> {
    let mut positions = Vec::new();
    let direction = if clockwise { -1.0 } else { 1.0 };

    // Convert stop angle from degrees to radians
    let stop_angle_radians = stop_angle_degrees.to_radians();
    
    // Calculate the number of intervals based on 5-degree increments
    let num_intervals = (stop_angle_degrees.abs() / 5.0).ceil() as i32;
    
    // Calculate positions along the circular path at 5-degree intervals
    for interval in 0..=num_intervals {
        let angle = (interval as f64 * 5.0).to_radians(); // Convert interval to radians
        let x = center.0 + radius * angle.cos();
        let y = center.1 + radius * angle.sin();
        positions.push((x, y));
    }

    positions
}