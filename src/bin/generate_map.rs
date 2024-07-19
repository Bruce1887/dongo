use dongo::map_generator::*;

fn main() {
    let mut mapgen = MapGenerator::new(dongo::common::MAP_SIZE);
    mapgen.define_parameters(ColorMode::HeightMap);

    println!("Writing to file...");
    mapgen.write_to_file();
}
