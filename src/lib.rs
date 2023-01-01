mod generator;

// public で公開
pub fn print_random_number() {
    let n = generator::gen_ran();
    println!("Random u8: {}", n);
}
