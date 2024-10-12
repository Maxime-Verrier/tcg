use bevy::prelude::Entity;
use synctree::{Node, NodeArena};

#[derive(Debug)]
pub struct Tree {
    tree: NodeArena<Entity>,
    current_leaf: Option<Node<Entity>>,
}

impl Tree {
    pub fn new(card: Entity) -> Self {
        let tree = NodeArena::new();
        let current_leaf = Node::new(card, &tree);

        Self {
            tree,
            current_leaf: Some(current_leaf),
        }
    }

    pub fn push_card(&mut self, card: Entity) {
        if let Some(current_leaf) = &self.current_leaf {
            current_leaf.append(&Node::new(card, &self.tree), &self.tree);
        } else {
            self.current_leaf = Some(Node::new(card, &self.tree));
        }
    }

    pub fn pop_card(&mut self) {
        if let Some(current_leaf) = &self.current_leaf {
            self.current_leaf = current_leaf.parent(&self.tree);
        }
    }
}
