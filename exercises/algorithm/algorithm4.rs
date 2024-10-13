/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/

use std::cmp::Ordering;
use std::fmt::Debug;


#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        //TODO
        match self.root {
            Some(ref mut node) => node.insert(value), // 如果树不为空，递归调用节点的插入方法
            None => {
                // 如果树为空，创建一个新的根节点
                self.root = Some(Box::new(TreeNode::new(value)));
            }
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        //TODo  
        self.search_node(&self.root, value)
    }
    fn search_node(&self, node: &Option<Box<TreeNode<T>>>, value: T) -> bool {
        match node {
            Some(ref boxed_node) => match value.cmp(&boxed_node.value) {
                Ordering::Less => self.search_node(&boxed_node.left, value), // 在左子树中查找
                Ordering::Greater => self.search_node(&boxed_node.right, value), // 在右子树中查找
                Ordering::Equal => true, // 找到目标值
            },
            None => false, // 节点为空，返回 false
        }
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        //TODO
        match value.cmp(&self.value) {
            Ordering::Less => {
                // 如果插入值小于当前节点值，插入到左子树
                if let Some(ref mut left) = self.left {
                    left.insert(value); // 递归调用插入函数
                } else {
                    self.left = Some(Box::new(TreeNode::new(value))); // 如果左子节点为空，创建一个新节点
                }
            }
            Ordering::Greater => {
                // 如果插入值大于当前节点值，插入到右子树
                if let Some(ref mut right) = self.right {
                    right.insert(value); // 递归调用插入函数
                } else {
                    self.right = Some(Box::new(TreeNode::new(value))); // 如果右子节点为空，创建一个新节点
                }
            }
            Ordering::Equal => {
                // 如果插入值等于当前节点值，不插入重复值
                println!("Value already exists in the tree.");
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        
        assert_eq!(bst.search(1), false);

        
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        
        bst.insert(1);
        bst.insert(1);

        
        assert_eq!(bst.search(1), true);

        
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


