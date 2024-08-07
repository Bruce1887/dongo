use noise::NoiseFn;

fn main(){

    let perlin = noise::Perlin::new(2);
    dbg!(perlin.get([0.1]));
    dbg!(perlin.get([1.0]));
    dbg!(perlin.get([1.1]));
    dbg!(perlin.get([100.0]));
    dbg!(perlin.get([100.1]));
    dbg!(perlin.get([5.000000001]));
    dbg!(perlin.get([6.1]));
}