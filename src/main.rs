pub mod dom;
pub mod html;

fn main() {
    let node = html::parse("<html><body></body><h1>Title</h1><div>Hello</div></html>".to_string());
    println!("{:?}", node)
}
