mod hash;

fn main() {
    let input_text = "Hello, world!";
    let hash = hash::get_sha256_hash(input_text);
    println!("{}", hash);
}
