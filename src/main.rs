
pub mod dom;

fn main() {
    let node = dom::text(String::from("Hello"));
    println!("{:?}", node)
}
