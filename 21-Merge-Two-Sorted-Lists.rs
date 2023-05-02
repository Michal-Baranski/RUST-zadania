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
    pub fn merge_two_lists(mut list1: Option<Box<ListNode>>, mut list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        
        let mut dummy_head = ListNode { val: 0, next: None };
        let mut tail = &mut dummy_head;

        while let (Some(node1), Some(node2)) = (list1.as_ref(), list2.as_ref())
        {
            if node1.val < node2.val
            {
                tail.next = Some(Box::new(ListNode
                {
                    val: node1.val,
                    next: None,
                }));
                list1 = node1.next.clone();
            }
            else
            {
                tail.next = Some(Box::new(ListNode
                {
                    val: node2.val,
                    next: None,
                }));
                list2 = node2.next.clone();
            }
            tail = tail.next.as_mut().unwrap();
        }

        tail.next = if list1.is_some()
        {
            list1
        }
        else
        {
            list2
        };

        dummy_head.next
    }
}

/*
You are given the heads of two sorted linked lists list1 and list2.

Merge the two lists in a one sorted list. The list should be made by splicing together the nodes of the first two lists.

Return the head of the merged linked list.

Example 1:

Input: list1 = [1,2,4], list2 = [1,3,4]
Output: [1,1,2,3,4,4]
Example 2:

Input: list1 = [], list2 = []
Output: []
Example 3:

Input: list1 = [], list2 = [0]
Output: [0]
*/
