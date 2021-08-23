use std::collections::HashMap;

pub struct MDXAST<'a> {
    pub head: String,
    pub nodes: Vec<MDXNode<'a>>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MDXNode<'a> {
    pub node_type: &'a str,
    pub children: Vec<MDXNode<'a>>,
    pub attrs: HashMap<String, String>,
    pub text: &'a str,
}

impl<'a> MDXNode<'a> {
    pub fn new(node_type: &'a str, text: &'a str) -> Self {
        MDXNode {
            node_type,
            children: Vec::new(),
            attrs: HashMap::new(),
            text,
        }
    }
}
