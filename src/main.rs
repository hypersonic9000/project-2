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

/// Function to read motions from a file
///
/// This function reads motions from a file specified by the given file path.
/// The file should contain commands in the following format:
/// - "LIN (x1, y1, z1) to (x2, y2, z2)" for linear motion
/// - "CW (x, y) radius r stop_angle a" or "CCW (x, y) radius r stop_angle a" for rotational motion
///
/// # Arguments
///
/// * `file_path` - The path to the file containing motion commands
///
/// # Returns
///
/// A Result containing a vector of Motion enums if successful, or an IO error otherwise.
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
                        // Calculate and print the positions for linear motion
                        let positions = linear_motion_calculate(linear_motion.start, linear_motion.end);
                        for position in positions {
                            println!("{}", position);
                        }
                    }
                    // Handle rotational motion
                    Motion::Rotational(rotational_motion) => {
                        println!("Rotational Motion: {:?}", rotational_motion);
                        // Calculate and print the positions for rotational motion
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


/// Function to calculate positions for linear motion
///
/// Given a start position and an end position, this function calculates
/// intermediate positions in a straight line between the start and end points.
/// The positions are calculated with one-unit increments.
///
/// # Arguments
///
/// * `start` - The starting position (x, y, z)
/// * `end` - The ending position (x, y, z)
///
/// # Returns
///
/// A vector of strings containing the calculated positions.
fn linear_motion_calculate(start: (f64, f64, f64), end: (f64, f64, f64)) -> Vec<String> {
    // Calculate the total change in each dimension
    let dx = end.0 - start.0;
    let dy = end.1 - start.1;
    let dz = end.2 - start.2;

    // Determine the maximum magnitude of change
    let max_delta = dx.abs().max(dy.abs()).max(dz.abs());

    // Determine the number of steps
    let num_steps = (max_delta.abs() + 1.0).ceil() as usize;

    // Print the delta steps for debugging
    println!("Delta steps: dx={}, dy={}, dz={}", dx, dy, dz);
    // Print the number of steps for debugging
    println!("Number of steps: {}", num_steps);

    // Calculate step increments for each dimension
    let dx_step = if num_steps != 0 { dx / num_steps as f64 } else { 0.0 };
    let dy_step = if num_steps != 0 { dy / num_steps as f64 } else { 0.0 };
    let dz_step = if num_steps != 0 { dz / num_steps as f64 } else { 0.0 };

    // Generate positions for each step
    let mut positions = Vec::new();
    for i in 1..=num_steps {
        let x = start.0 + dx_step * i as f64;
        let y = start.1 + dy_step * i as f64;
        let z = start.2 + dz_step * i as f64;
        positions.push(format!("{:.2}, {:.2}, {:.2}", x, y, z));
    }

    positions
}

/// Function to calculate positions for rotational motion
///
/// Given the parameters of a rotational motion (center, radius, clockwise,
/// stop angle), this function calculates the positions along the arc of the
/// rotation. Positions are calculated at 5-degree intervals.
///
/// # Arguments
///
/// * `rotational_motion` - A struct containing the parameters of the rotational motion.
///
/// # Returns
///
/// A vector of tuples containing the calculated (x, y) positions.
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

#[cfg(test)]
mod tests {
    // Import necessary items from the parent module
    use super::*;

    /// Test the `linear_motion_calculate` function.
    #[test]
    fn test_linear_motion_calculate() {
        // Test linear motion calculation function
        let start = (0.0, 0.0, 0.0);
        let end = (3.0, 4.0, 5.0);
        let positions = linear_motion_calculate(start, end);
        assert_eq!(positions.len(), 7); // Adjusted for inclusive start and end points
        assert_eq!(positions[0], "0.00, 0.00, 0.00"); // Adjusted start position
        assert_eq!(positions[6], "3.00, 4.00, 5.00"); // Check last position
    }

    /// Test the `rotational_motion_calculate` function.
    #[test]
    fn test_rotational_motion_calculate() {
        // Test rotational motion calculation function
        let rotational_motion = RotationalMotion {
            center: (0.0, 0.0),
            radius: 5.0,
            clockwise: true,
            stop_angle: 90.0,
        };
        let positions = rotational_motion_calculate(rotational_motion);
        assert_eq!(positions.len(), 21); // Adjusted expected number of positions
        assert_eq!(positions[0], (5.00, 0.00)); // Check first position
        assert_eq!(positions[20], (0.00, 5.00)); // Check last position
    }

   /// Test the `read_file` function.
#[test]
fn test_read_file() {
    // Test reading motions from a file
    let result = read_file("test.cmmd");
    assert!(result.is_ok()); // Check if reading succeeds
    let motions = result.unwrap();
    assert_eq!(motions.len(), 8); // Check number of motions read
    // Add more specific checks if needed
    }
}