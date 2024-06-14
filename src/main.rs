pub mod css;
pub mod dom;
pub mod html;

fn main() {
    let node = html::parse(
        "<html><body id='main'><h1>Title</h1><div id='hello'>Hello</div></body></html>".to_string(),
    );
    println!("{node}")
}
