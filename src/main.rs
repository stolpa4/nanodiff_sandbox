mod sandbox;
mod ad;
mod example1;

fn main() {
    println!(" === Sandbox ===");
    sandbox::run();

    println!(" === Example 1 === ");
    example1::run();
}
