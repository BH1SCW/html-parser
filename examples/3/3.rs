use html_parser::{Dom, Node, Result};

fn main() -> Result<()> {
    let html = include_str!("./test.html");
    let dom = Dom::parse(html)?;
    let iter = dom.children.get(0).unwrap().into_iter();

    let hrefs = iter.filter_map(|item| match item {
        Node::Element(ref element) if element.name == "a" => element.attributes["href"].clone(),
        _ => None,
    });

//    println!("\nThe following links where found:");
//    for (index, href) in hrefs.enumerate() {
//        println!("{}: {}", index + 1, href)
//    }

    for (_, href) in hrefs.enumerate() {
        if href.starts_with("/rpms") {
            let pkg_name = &href[6..];
            println!("{}", pkg_name);
        }
    }
//    let hrefs = iter.filter_map(|item| match item {
//        Node::Element(ref element) if element.name == "div" => element.attributes["title"].clone(),
//        _ => None,
//    });
//
//    println!("\nThe following links where found:");
//    for (index, href) in hrefs.enumerate() {
//        println!("{}: {}", index + 1, href)
//    }

    Ok(())
}
