// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
       
        match root {
            Some(node) => {
                //mamy niepusty węzeł drzewa.
                //uzyskujemy dostęp do lewego i prawego dziecka bieżącego węzła przy użyciu `node.borrow().left` i `node.borrow().right`.
                //rekurencyjne wywołania funkcji `max_depth` do obliczenia maksymalnej głębokości lewego i prawego poddrzewa.
                let left_depth = Solution::max_depth(node.borrow().left.clone());
                let right_depth = Solution::max_depth(node.borrow().right.clone());

                //zwracamy maksymalną głębokość bieżącego poddrzewa, dodając 1 do maksymalnej głębokości jego lewego i prawego poddrzewa przy   użyciu `1 + left_depth.max(right_depth)`.
                1 + left_depth.max(right_depth)
            }

            //mamy pusty węzeł drzewa wiec zwracamy 0
            None => 0,
        }
    
    }
}

/*
Given the root of a binary tree, return its maximum depth.

A binary tree's maximum depth is the number of nodes along the longest path from the root node down to the farthest leaf node.
*/
