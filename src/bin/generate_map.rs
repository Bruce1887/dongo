use dongo::common::MAP_COLOR_MODE;
use dongo::map_generator::*;

fn main() {
    
    let mut mapgen = MapGenerator::new(dongo::common::MAP_SIZE);
    mapgen.define_parameters(MAP_COLOR_MODE);
    
    println!("Writing to file...");
    mapgen.write_to_file();
    println!("Struct fields written to file");
    
}
