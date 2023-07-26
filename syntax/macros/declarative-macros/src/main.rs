fn main() {
    let v: Vec<u32> = declarative_macros::vec![1, 2, 3];
    println!("v: {:?}", v);
}
