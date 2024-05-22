use operations::math::{sum, subtract};
use rand::random;
// Essas funções estão no mesmo modulo
// Principio do encapsulamento
mod operations;

fn main() {
    println!("Sum: {}", sum(10, 5));
    println!("Subtract: {}", subtract(10, 5));

    println!("Random number between 1 and 100: {}", random::<i32>());
}
