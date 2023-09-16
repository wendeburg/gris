use std::collections::HashMap;
use std::fmt::Debug;
use crate::routing::{Callable};

#[derive(Debug, PartialEq)]
pub struct PathTree {
    root: PathNode
}

#[derive(Debug)]
struct PathNode {
    pub path_part: String,
    pub children: HashMap<String, PathNode>,
    pub handler: Option<Box<dyn Callable>>
}

impl PathNode {
    fn new(path: &str) -> PathNode {
        PathNode {
            path_part: path.to_owned(),
            children: HashMap::new(),
            handler: None
        }
    }

    pub fn add_handler(&mut self, handler: impl Callable + 'static) {
        self.handler = Some(Box::new(handler));
    }
}

impl PartialEq for PathNode {
    fn eq(&self, other: &Self) -> bool {
        return self.path_part == other.path_part && self.children == other.children;
    }

    fn ne(&self, other: &Self) -> bool {
        return !self.eq(other);
    }
}

impl PathTree {
    pub fn new() -> PathTree {
        PathTree {
            root: PathNode::new("/")
        }
    }

    pub fn insert(&mut self, path: &str, handler: impl Callable + 'static) {
        let mut current_node= &mut self.root;

        let path_parts = path.split("/").filter(|&element| !element.is_empty());

        for path in path_parts {
            if current_node.path_part == path {
                continue;
            }

            current_node = current_node.children.entry(path.to_owned()).or_insert(PathNode::new(path));
        }

        current_node.add_handler(handler);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insertion_1() {
        let path = "/api/users/:user_id";

        let mut user_id_node = PathNode::new(":user_id");
        user_id_node.handlers.push(|req, res| {
            println!("hello!");
        });
        let mut users_node = PathNode::new("users");
        users_node.children.insert(":user_id".to_owned(), user_id_node);
        let mut api_node = PathNode::new("api");
        api_node.children.insert("users".to_owned(), users_node);
        let mut expected = PathTree::new();
        expected.root.children.insert("api".to_owned(), api_node);

        let mut actual = PathTree::new();
        actual.insert(path, |req, res| {
            println!("hello!");
        });

        assert_eq!(expected, actual);
    }

    #[test]
    fn insertion_2() {
        let path = "/";
        let expected = PathTree::new();

        let mut actual = PathTree::new();
        actual.insert(path, |req, res| {
            println!("hello!");
        });

        assert_eq!(expected, actual);
    }

    #[test]
    fn insertion_3() {
        let path = "/api/users/:user_id/profile_picture";

        let profile_picture_node = PathNode::new("profile_picture");
        let mut user_id_node = PathNode::new(":user_id");
        user_id_node.children.insert("profile_picture".to_owned(), profile_picture_node);
        let mut users_node = PathNode::new("users");
        users_node.children.insert(":user_id".to_owned(), user_id_node);
        let mut api_node = PathNode::new("api");
        api_node.children.insert("users".to_owned(), users_node);
        let mut expected = PathTree::new();
        expected.root.children.insert("api".to_owned(), api_node);

        let mut actual = PathTree::new();
        actual.insert(path, |req, res| {
            println!("hello!");
        });

        assert_eq!(expected, actual);
    }

    #[test]
    fn insertion_4() {
        let path = "/api/users/:user_id/:other_parameter";

        let profile_picture_node = PathNode::new(":other_parameter");
        let mut user_id_node = PathNode::new(":user_id");
        user_id_node.children.insert(":other_parameter".to_owned(), profile_picture_node);
        let mut users_node = PathNode::new("users");
        users_node.children.insert(":user_id".to_owned(), user_id_node);
        let mut api_node = PathNode::new("api");
        api_node.children.insert("users".to_owned(), users_node);
        let mut expected = PathTree::new();
        expected.root.children.insert("api".to_owned(), api_node);

        let mut actual = PathTree::new();
        actual.insert(path, |req, res| {
            println!("hello!");
        });

        assert_eq!(expected, actual);
    }

    #[test]
    fn insertion_5() {
        let path = "/api/";
        let other_path = "/api";

        let mut right = PathTree::new();
        right.insert(path, |req, res| {
            println!("hello!");
        });

        let mut left = PathTree::new();
        left.insert(other_path, |req, res| {
            println!("hello!");
        });

        assert_eq!(left, right);
    }
}
