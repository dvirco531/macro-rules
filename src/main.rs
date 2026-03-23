#[macro_use]
mod max;
#[macro_use]
mod calculate;
fn main() {
    println!("{}", max!(1, 200, 3, 1));
    println!("{}", calculate!(add, 1, 2));
    println!("{}", calculate!(subtract, 1, 2));
    println!("{}", calculate!(multiply, 1, 2));
    println!("{}", calculate!(divide, 1, 2));
}
