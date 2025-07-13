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
fn main() {

    add_two(l1, l2, 0);

    fn add_two(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
        carry: i32,
    ) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (Some(mut n1), Some(mut n2)) => {
                let sum = n1.val + n2.val + carry;
                let mut node = Box::new(ListNode::new(sum % 10));
                node.next = Self::add_two(n1.next, n2.next, sum / 10);
                Some(node)
            }
            (Some(mut n1), None) => {
                let sum = n1.val + carry;
                let mut node = Box::new(ListNode::new(sum % 10));
                node.next = Self::add_two(n1.next, None, sum / 10);
                Some(node)
            }
            (None, Some(mut n2)) => {
                let sum = n2.val + carry;
                let mut node = Box::new(ListNode::new(sum % 10));
                node.next = Self::add_two(None, n2.next, sum / 10);
                Some(node)
            }
            (None, None) => {
                if carry > 0 {
                    Some(Box::new(ListNode::new(carry)))
                } else {
                    None
                }
            }
        }
    }
}