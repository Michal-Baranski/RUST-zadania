// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut arr: Vec<i32> = Vec::new();

        while let Some(mut node) = head
        {
            arr.push(node.val);
            head = node.next.take();
        }

        arr.sort();

        let mut dummy_head = Box::new(ListNode {val: 0, next: None});
        let mut tail = &mut dummy_head;

        for &val in &arr
        {
            tail.next = Some(Box::new(ListNode {val, next: None}));
            tail = tail.next.as_mut().unwrap();
        }
        dummy_head.next
    }
}

/*
Given the head of a linked list, return the list after sorting it in ascending order.

Example 1:
Input: head = [4,2,1,3]
Output: [1,2,3,4]

Example 2:
Input: head = [-1,5,3,4,0]
Output: [-1,0,3,4,5]
*/
