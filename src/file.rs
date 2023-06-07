use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
    ops::Range,
};

use crate::node::Node;

pub fn lines(path: &str) -> Result<Vec<String>> {
    let file = File::open(path)?;
    let buf = BufReader::new(file);
    Ok(buf.lines().collect::<Result<_>>().unwrap())
}

pub fn indent_level(line: &str) -> usize {
    line.len() - line.trim_start().len()
}

pub fn split_by_section(lines: Vec<String>) -> Vec<Node> {
    let mut nodes: Vec<Node> = vec![];
    let start_indent = indent_level(lines.get(0).unwrap());
    let mut node: Node;
    let mut next_skip = 0;

    for (i, line) in lines.iter().enumerate() {
        if i >= next_skip {
            let indent = indent_level(line);
            if indent < start_indent {
                break;
            }
            if indent == start_indent {
                node = Node::new(line);
                if indent == start_indent
                    && i < lines.len() - 1
                    && indent < indent_level(lines.get(i + 1).unwrap())
                {
                    node.add_children(split_by_section(lines[(i + 1)..].to_vec()));
                    next_skip = i + node.child_count() + 1;
                }
                nodes.push(node);
            }
        }
    }
    return nodes;
}
