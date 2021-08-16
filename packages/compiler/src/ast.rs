use std::collections::HashMap;

pub struct MDXAST {
    head: String,
    nodes: Vec<MDXNode>,
}

pub struct MDXNode {
    id: String,
    node_type: String,
    children: Vec<MDXNode>,
    text: String,
    attrs: HashMap<String, String>,
}
