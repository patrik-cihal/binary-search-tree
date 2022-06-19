use std::cmp;
use std::fmt;

pub struct BinaryTree<T: std::cmp::PartialOrd> {
    root: Option<Box<BinaryNode<T>>>
}

impl<T: std::cmp::PartialOrd> Default for BinaryTree<T> {
    fn default() -> Self {
        Self { 
            root: None 
        }
    }
}

enum DeleteResult<T: cmp::PartialOrd> {
    Found,
    NotFound,
    Pending(Option<Box<BinaryNode<T>>>),
}

impl<T: cmp::PartialOrd + Clone + ToOwned + fmt::Debug> BinaryTree<T> {
    pub fn insert(&mut self, value: T) -> bool {
        if self.root.is_none() {
            self.root = Some(Box::new(BinaryNode::<T>::new(value)));
            return true;
        }else {
            return self.root.as_mut().unwrap().insert(value);
        }
    }

    pub fn find(&self, value: T) -> bool {
        if self.root.is_none() {
            return false;
        }
        return self.root.as_ref().unwrap().find(value);
    }

    pub fn delete(&mut self, value: T) -> bool {
        if let Some(root) = &mut self.root {
            match root.delete(value) {
                DeleteResult::Found => return true,
                DeleteResult::NotFound => return false,
                DeleteResult::Pending(rn) => {
                    self.root = rn;
                    return true;
                }
            }; 
        };
        return false
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        if self.root.is_none() {
            return;
        }
        self.traverse(&self.root.as_ref().unwrap(), 0, &mut |cn: &BinaryNode<T>, level: usize| {
            let level_str = "-".repeat(level+1);
            println!("{:?} {:?}", level_str, cn.value);
            return true;
        });
    }

    fn traverse<U>(&self, node: &BinaryNode<T>, level: usize, callback: &mut U) where U: FnMut(&BinaryNode<T>, usize) -> bool {
        if callback(node, level) {
            if node.left.is_some() {
                self.traverse(&node.left.as_ref().unwrap(), level+1, callback)
            }
            if node.right.is_some() {
                self.traverse(&node.right.as_ref().unwrap(), level+1, callback);
            }
        }
    }
}

#[derive(Clone)]
struct BinaryNode<T: cmp::PartialEq + std::cmp::PartialOrd> {
    value: T,
    left: Option<Box<BinaryNode<T>>>,
    right: Option<Box<BinaryNode<T>>>,
}

impl<T: cmp::PartialOrd + Clone + ToOwned + fmt::Debug> BinaryNode<T> {
    fn new(value: T) -> Self {
        Self { 
            value,
            left: None, 
            right: None 
        }
    }
    fn find(&self, value: T) -> bool {
        if value == self.value {
            return true;
        }
        else if value > self.value {
            return self.right.is_some() && self.right.as_ref().unwrap().find(value);
        }
        return self.left.is_some() && self.left.as_ref().unwrap().find(value);
    }
    fn insert(&mut self, value: T) -> bool {
        if value == self.value {
            return false;
        }
        else if value < self.value {
            if self.left.is_none() {
                self.left = Some(Box::new(BinaryNode::<T>::new(value)));
                return true;
            }
            else {
                return self.left.as_mut().unwrap().insert(value);
            }
        } else {
            if self.right.is_none() {
                self.right = Some(Box::new(BinaryNode::<T>::new(value)));
                return true;
            }
            else {
                return self.right.as_mut().unwrap().insert(value);
            }
        }
    }
    fn delete(&mut self, value: T) -> DeleteResult<T> {
        if self.value == value {
            if self.right.is_some() && self.left.is_some() {
                let rn = self.pop_right();
                return DeleteResult::Pending(Some(rn));
            }
            else if self.right.is_some() {
                return DeleteResult::Pending(self.right.to_owned());
            }
            else if self.left.is_some() {
                return DeleteResult::Pending(self.left.to_owned());
            }
            return DeleteResult::Pending(None);
        }else if value < self.value {
            if self.left.is_some() {
                let dr = self.left.as_mut().unwrap().delete(value);
                match dr {
                    DeleteResult::Pending(rn) => {
                        self.left = rn;
                        DeleteResult::Found
                    },
                    _ => dr
                }; 
            }
            return DeleteResult::NotFound;
        }else {
            if self.right.is_some() {
                let dr = self.right.as_mut().unwrap().delete(value);
                match dr {
                    DeleteResult::Pending(rn) => {
                        self.right = rn;
                        DeleteResult::Found
                    },
                    _ => dr
                }; 
            }
            return DeleteResult::NotFound;
        }
    }
    fn pop_right(&mut self) -> Box<BinaryNode::<T>> {
        if self.right.as_ref().unwrap().right.is_none() {
            let rn = self.right.as_ref().unwrap().to_owned();
            self.right = self.right.as_ref().unwrap().left.to_owned();
            return rn;
        }
        return self.right.as_mut().unwrap().pop_right();
    }
}
