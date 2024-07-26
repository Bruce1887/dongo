//! All of these are straight up botted.

use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

pub fn center_obj_vertices(input_path: &str, output_path: &str) -> io::Result<()> {
    assert_ne!(input_path, output_path);

    let input_file = File::open(input_path)?;
    let reader = BufReader::new(input_file);

    let mut vertices = Vec::new();
    let mut lines = Vec::new();

    // Read and parse the .obj file
    for line in reader.lines() {
        let line = line?;
        if line.starts_with("v ") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() == 4 {
                let x: f32 = parts[1].parse().unwrap();
                let y: f32 = parts[2].parse().unwrap();
                let z: f32 = parts[3].parse().unwrap();
                vertices.push((x, y, z));
            }
        }
        lines.push(line);
    }

    // Calculate the centroid of the vertices
    let num_vertices = vertices.len() as f32;
    let (mut sum_x, mut sum_y, mut sum_z) = (0.0, 0.0, 0.0);
    for (x, y, z) in &vertices {
        sum_x += x;
        sum_y += y;
        sum_z += z;
    }
    let centroid = (sum_x / num_vertices, sum_y / num_vertices, sum_z / num_vertices);

    // Translate vertices to center around the origin
    let centered_vertices: Vec<(f32, f32, f32)> = vertices
        .iter()
        .map(|(x, y, z)| (x - centroid.0, y - centroid.1, z - centroid.2))
        .collect();

    // Write the modified vertices and other lines back to a new .obj file
    let mut output_file = File::create(output_path)?;
    let mut vertex_index = 0;
    for line in lines {
        if line.starts_with("v ") {
            let (x, y, z) = centered_vertices[vertex_index];
            writeln!(output_file, "v {} {} {}", x, y, z)?;
            vertex_index += 1;
        } else {
            writeln!(output_file, "{}", line)?;
        }
    }

    Ok(())
}


pub fn resize_obj_vertices(input_path: &str, output_path: &str) -> io::Result<()> {
    let input_file = File::open(input_path)?;
    let reader = BufReader::new(input_file);

    let mut vertices = Vec::new();
    let mut lines = Vec::new();

    // Read and parse the .obj file
    for line in reader.lines() {
        let line = line?;
        if line.starts_with("v ") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() == 4 {
                let x: f32 = parts[1].parse().unwrap();
                let y: f32 = parts[2].parse().unwrap();
                let z: f32 = parts[3].parse().unwrap();
                vertices.push((x, y, z));
            }
        }
        lines.push(line);
    }

    // Find the bounding box of the vertices
    let (mut min_x, mut min_y, mut min_z) = (f32::MAX, f32::MAX, f32::MAX);
    let (mut max_x, mut max_y, mut max_z) = (f32::MIN, f32::MIN, f32::MIN);
    for (x, y, z) in &vertices {
        if *x < min_x { min_x = *x; }
        if *y < min_y { min_y = *y; }
        if *z < min_z { min_z = *z; }
        if *x > max_x { max_x = *x; }
        if *y > max_y { max_y = *y; }
        if *z > max_z { max_z = *z; }
    }

    // Calculate the scaling factor
    let scale_x = 2.0 / (max_x - min_x);
    let scale_y = 2.0 / (max_y - min_y);
    let scale_z = 2.0 / (max_z - min_z);
    let scale = scale_x.min(scale_y).min(scale_z);

    // Scale vertices to fit within (-1, -1, -1) to (1, 1, 1)
    let scaled_vertices: Vec<(f32, f32, f32)> = vertices
        .iter()
        .map(|(x, y, z)| ((x - min_x) * scale - 1.0, (y - min_y) * scale - 1.0, (z - min_z) * scale - 1.0))
        .collect();

    // Write the scaled vertices and other lines back to a new .obj file
    let mut output_file = File::create(output_path)?;
    let mut vertex_index = 0;
    for line in lines {
        if line.starts_with("v ") {
            let (x, y, z) = scaled_vertices[vertex_index];
            writeln!(output_file, "v {} {} {}", x, y, z)?;
            vertex_index += 1;
        } else {
            writeln!(output_file, "{}", line)?;
        }
    }

    Ok(())
}
