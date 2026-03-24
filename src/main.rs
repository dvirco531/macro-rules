/// Main for macros simple tests with prints to both max and calculate macro
/// 
#[macro_use]
mod max;
#[macro_use]
mod calculate;
fn main() {
    println!("{}", max!(1, 200, 3, 1));
    println!("{}", calculate!(add, 1i32, 2i32));
    println!("{}", calculate!(subtract, 1i32, 2i32));
    println!("{}", calculate!(multiply, 1i32, 2i32));
    println!("{}", calculate!(divide, 1i32, 2i32));
}
