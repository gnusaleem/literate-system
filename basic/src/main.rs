use rand::Rng;

fn main() {
    let a = 5;
    let b = 6;

    // Initialize the generator
    let mut rng = rand::rng();

    // Use r#gen() to avoid the 'gen' keyword conflict
    let sec_num: i32 = rng.r#gen();

    println!("sec number = {sec_num}");
    println!("a = {a} and b = {b}");
    println!("a = {a} and b + 2 = {}", b + 2);
}
