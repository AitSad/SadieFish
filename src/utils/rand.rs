use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();

    // Generate a random integer between 1 and 100
    let random_int = rng.gen_range(1, 101);
    println!("Random integer: {}", random_int);

    // Generate a random boolean
    let random_bool = rng.gen_bool(0.5);
    println!("Random boolean: {}", random_bool);

    // Generate a random float between 0 and 1
    let random_float = rng.gen_range(0.0, 1.0);
    println!("Random float: {}", random_float);
}
