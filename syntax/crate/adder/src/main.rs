fn main() {
    println!("Hello, Adder!");
    let arg = 5;
    let answer = adder::add_one(arg);
    println!("answer = {}", answer);
}
