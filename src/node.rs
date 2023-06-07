#[derive(Clone)]
pub struct Node {
    name: String,
    self_closing: bool,
    properties: Option<Vec<Property>>,
    content: Option<String>,
    children: Vec<Node>,
    pub raw: String
}

#[derive(Clone)]
pub struct Property {
    name: String,
    value: Option<String>,
}

impl Node {
    pub fn new(raw: &str) -> Node {
        Node { 
            name: String::new(),
            self_closing: false,
            properties: None,
            content: None,
            children: vec![],
            raw: raw.trim().to_string()
        }
    }
    
    pub fn print(&self, prefix: String) {
        println!("{}{}", prefix, self.raw);
        let mut pre = prefix.clone();
        pre.push_str("  ");
        for child in self.children.clone() {
            child.print(pre.clone());
        }
    }

    pub fn add_children(&mut self, nodes: Vec<Node>) {
        for node in nodes {
            self.children.push(node);
        }
    }

    pub fn child_count(&mut self) -> usize {
        self.children.len()
    }
}
