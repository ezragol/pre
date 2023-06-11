use std::cmp::min;

use ::regex::Regex;
use lazy_static::lazy_static;

#[derive(Clone)]
pub struct Node {
    name: String,
    properties: Vec<Property>,
    content: String,
    pub children: Vec<Node>,
    pub raw: String,
}

#[derive(Clone)]
pub struct Property {
    name: String,
    value: String,
}

pub fn tail(string: &str) -> String {
    string[1..].to_string()
}

pub fn head(string: &str) -> String {
    string[..string.len() - 1].to_string()
}

impl Node {
    pub fn new(raw: &str) -> Node {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"((?:\.[^#:.\s]+)|(?:#[^#:.\s]+)|(?::[^#:\s]+)*)").unwrap();
        }
        let mut properties: Vec<Property> = vec![];
        let mut content = String::new();
        let mut name = String::new();

        let line = raw.trim();
        let splits = line.split_whitespace().collect::<Vec<&str>>();
        if splits.len() > 0 {
            let mut first_split = splits[0];

            let self_closing = first_split.find('!');
            if let Some(_) = self_closing {
                properties.push(Property::new("_selfclosing".to_string(), None));
                first_split = &first_split[..first_split.len() - 1];
            }

            let matches = RE.captures_iter(first_split);
            let mut count = 0;

            for capture in matches {
                let exact = capture.get(0).unwrap().as_str();
                if !exact.is_empty() && exact.len() > 1 {
                    count += 1;
                    let prop_name = if exact.starts_with('.') {
                        "class"
                    } else if exact.starts_with('#') {
                        "id"
                    } else if exact.starts_with(':') {
                        "name"
                    } else {
                        "_"
                    };

                    let mut already_exists = false;
                    for prop in &mut properties {
                        if prop.name == prop_name {
                            already_exists = true;
                            prop.value = format!("{} {}", prop.value, tail(exact));
                        }
                    }
                    if !already_exists {
                        properties.push(Property::new(prop_name.to_string(), Some(tail(exact))));
                    }
                }
            }

            if count > 0 {
                let len = first_split.len() - 1;
                let class_ind = first_split.find('.').unwrap_or(len);
                let id_ind = first_split.find('#').unwrap_or(len);
                let name_ind = first_split.find(':').unwrap_or(len);

                let end = min(class_ind, min(id_ind, name_ind));
                name = first_split[..end].to_string();
            } else {
                name = first_split.to_string();
            }

            if splits.len() > 1 {
                let mut next_prop = Property::new("".to_string(), None);
                let mut read_next = false;

                // take everything except first section before whitespace (node name)
                for split in splits[1..].into_iter() {
                    if !read_next && split.starts_with('[') {
                        next_prop.name = tail(split);
                        read_next = true;
                    } else if read_next {
                        if split.ends_with(']') {
                            next_prop.value = format!("{}{}", next_prop.value, head(split));
                            read_next = false;
                            properties.push(next_prop.clone());
                            next_prop.value = String::new();
                        } else {
                            next_prop.value = format!("{}{} ", next_prop.value, split);
                        }
                    } else {
                        content.push_str(split);
                    }
                }
            }
        }

        Node {
            name,
            properties,
            content,
            children: vec![],
            raw: line.trim().to_string(),
        }
    }

    pub fn print(&self, prefix: String) -> String {
        let mut full = String::new();
        let mut self_closing = false;

        if self.name.to_lowercase() == "doctype" {
            let to_print = format!("<!DOCTYPE {}>", self.content.to_uppercase());
            return to_print;
        }

        full.push('<');
        full.push_str(&self.name.to_lowercase());

        for prop in self.properties.clone() {
            if prop.name == "_selfclosing" {
                self_closing = true;
                continue;
            }

            full.push(' ');
            full.push_str(&prop.name.to_lowercase());

            if !prop.value.is_empty() {
                full.push('=');
                let wrapped = format!("\"{}\"", prop.value);
                full.push_str(&wrapped);
            }
        }

        if self_closing {
            full.push_str("/>\n");
        } else {
            let has_children = self.children.len() > 0;
            full.push('>');
            if has_children && !self.content.is_empty() {
                full.push_str(&format!("\n{}  {}\n", prefix, &self.content));
            } else if !self.content.is_empty() {
                full.push_str(&self.content);
            } else if has_children {
                full.push('\n');
            }

            for child in self.children.clone() {
                full.push_str(&child.print(format!("  {}", prefix)));
            }
            full.push_str(&format!(
                "{}</{}>\n",
                if has_children {
                    prefix.clone()
                } else {
                    String::new()
                },
                self.name.to_lowercase()
            ));
        }
        return format!("{}{}", prefix, full);
    }

    pub fn add_children(&mut self, nodes: Vec<Node>) {
        for node in nodes {
            self.children.push(node);
        }
    }
}

impl Property {
    pub fn new(name: String, value: Option<String>) -> Property {
        Property {
            name,
            value: value.unwrap_or(String::new()),
        }
    }
}
