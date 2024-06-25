use std::collections::{HashMap, HashSet};
use std::fmt;

pub type AttrMap = HashMap<String, String>;

#[derive(Debug)]
pub struct Node {
    pub children: Vec<Node>,
    pub node_type: NodeType,
}

#[derive(Debug)]
pub enum NodeType {
    Text(String),
    Element(ElementData),
}

#[derive(Debug)]
pub struct ElementData {
    pub tag_name: String,
    pub attributes: AttrMap,
}

impl ElementData {
    pub fn id(&self) -> Option<&String> {
        self.attributes.get("id")
    }

    pub fn classes(&self) -> HashSet<&str> {
        match self.attributes.get("class") {
            Some(classes) => classes.split(' ').collect(),
            None => HashSet::new(),
        }
    }
}

impl Node {
    pub fn text(data: String) -> Node {
        Node {
            children: Vec::new(),
            node_type: NodeType::Text(data),
        }
    }

    pub fn elem(name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
        Node {
            children: children,
            node_type: NodeType::Element(ElementData {
                tag_name: name,
                attributes: attrs,
            }),
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.node_type {
            NodeType::Element(data) => write!(f, "<{}>\n", data.tag_name)?,
            NodeType::Text(string) => write!(f, "{}\n", string)?,
        };

        for node in &self.children {
            write!(f, "{}", node)?;
        }

        match &self.node_type {
            NodeType::Element(data) => write!(f, "</{}>\n", data.tag_name),
            NodeType::Text(_) => write!(f, ""),
        }
    }
}
