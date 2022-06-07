use crate::chain::node::Node;

pub struct Interface {
    pub machina: Node,
}

impl Interface {
    pub fn new() -> Self {
        Self {
            machina: Node::new()
        }
    }
}