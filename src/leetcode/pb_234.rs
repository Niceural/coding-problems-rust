use std::collections::LinkedList;
// 234. Palindrome Linked List
// https://leetcode.com/problems/palindrome-linked-list/

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

// a -> b -> c -> c-> b -> a
pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
    match head {
        Some(node) => {
            if node.next == None { return true; }

            let mut prev = Some(slow);
            let mut curr = slow.next.as_ref();
            let mut next;
            loop {
                if let Some(mut node) = curr {
                    curr = &node.next;
                    node.next = prev;
                    prev = Some(node);
                }
            }
            return false;
        }
        None => return true,
    }

    fn find_middle_end(head: &Option<Box<ListNode>>) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
        let mut slow = head;
        let mut fast = head;

        loop {
            fast = match &fast.next.as_ref() {
                &Some(node) => node,
                None => break,
            };
            fast = match &fast.next.as_ref() {
                &Some(node) => node,
                None => break,
            };
            slow = &slow.next.as_ref().unwrap();
        }
    }
}