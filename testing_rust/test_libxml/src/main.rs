use libxml::parser::Parser;
use libxml::tree::*;

// apt install libxml2-dev

// dies with "killed" message

fn my_recurse(node: &Node) {
    match node.get_type().unwrap() {
        NodeType::ElementNode => {
            println!("Entering {}", node.get_name());
        }
        NodeType::TextNode => {
            println!("Text: {}", node.get_content());
        }
        _ => {}
    }

    let mut c: Option<Node> = node.get_first_child();
    while let Some(child) = c {
        my_recurse(&child);
        c = child.get_next_sibling();
    }

    if node.get_type().unwrap() == NodeType::ElementNode {
        println!("Leaving {}", node.get_name());
    }
}

fn main() {
    let parser = Parser::default();
    let doc = parser
        .parse_file("/home/spoot/Downloads/mame0241lx/mame0241.xml")
        .unwrap();
    let root = doc.get_root_element().unwrap();
    my_recurse(&root);
}
