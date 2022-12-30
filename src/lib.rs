mod list_to_tree;
use list_to_tree::IntoTree;

use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct Node {
    parent_id: i32,
    id: i32,
    content: i32,
    children: Vec<Node>,
}

impl Node {
    pub fn new(parent_id: i32, id: i32, content: i32) -> Self {
        Self {
            parent_id,
            id,
            content,
            children: Vec::with_capacity(2)
        }
    }
}

impl IntoTree for Node {
    type Output = Self;

    fn get_id(&self) -> i32 {
        self.id
    }

    fn get_parent_id(&self) -> i32 {
        self.parent_id
    }

    fn convert(&self, children: Vec<Self>) -> Self {
        Self {
            id: self.id,
            parent_id: self.parent_id,
            content: self.content,
            children,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use list_to_tree::convert_to_tree;

    #[test]
    fn test_name() {
        let root_menus = vec![Node::new(0, 1, 101), Node::new(0, 2, 102)];
        let sub_menus = vec![
            Node::new(1, 3, 201),
            Node::new(1, 4, 202),
            Node::new(2, 5, 203),
            Node::new(3, 6, 204),
            Node::new(6, 7, 205),
        ];

        let result = convert_to_tree(&root_menus, &sub_menus);

        // println!("{}", serde_json::to_string(&result).unwrap());

        // let s = r#"[{"id":1,"content":101,"parent_id":0,"children":[{"id":3,"content":201,"parent_id":1,"children":[{"id":6,"content":204,"parent_id":3,"children":[{"id":7,"content":205,"parent_id":6,"children":[]}]}]},{"id":4,"content":202,"parent_id":1,"children":[]}]},{"id":2,"content":102,"parent_id":0,"children":[{"id":5,"content":203,"parent_id":2,"children":[]}]}]"#;
        let s = r#"[{"parent_id":0,"id":1,"content":101,"children":[{"parent_id":1,"id":3,"content":201,"children":[{"parent_id":3,"id":6,"content":204,"children":[{"parent_id":6,"id":7,"content":205,"children":[]}]}]},{"parent_id":1,"id":4,"content":202,"children":[]}]},{"parent_id":0,"id":2,"content":102,"children":[{"parent_id":2,"id":5,"content":203,"children":[]}]}]"#;
        assert_eq!(s, serde_json::to_string(&result).unwrap());
    }
}
