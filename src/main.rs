pub mod css;
pub mod dom;
pub mod html;

fn main() {
    let node = html::parse(
        "<html><body id='main'><h1>Title</h1><div id='hello'>Hello</div></body></html>".to_string(),
    );
    println!("{node}");

    let style = css::parse("#id {width: 15px; color: #ffffff; display: block;} .main, #ui, p {background: #ffffff; display: flex; margin: 10px;}". to_string());
    println!("{style}");
}
