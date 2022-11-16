// Definition for singly-linked list.
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

struct Solution {}

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        // let mut right: &ListNode = head.unwrap().as_ref();
        // let mut left: &ListNode = head.unwrap().as_ref();
        
        // // interval of n between left and right
        // for _ in 0..n {
        //     right = right.next.unwrap().as_ref();
        // }
        
        // // iterate until right is at the end of the list
        // loop {
        //     match (*right).next {
        //         Some(nxt) => right = nxt.as_ref(),
        //         None => break,
        //     }
        //     left = left.next.unwrap().as_ref();
        // }
        
        // // remove and return element
        // let el_to_return = left.next.unwrap();
        // left.next = el_to_return.next;
        
        
        // Some(el_to_return)
        let mut dummy = Box::new(ListNode {
            val: -1,
            next: head,
        });

        let mut right = dummy.clone();
        let mut left = dummy.as_mut();

        for _ in 0..n {
            right = right.next.unwrap();
        }

        while let Some(node) = right.next {
            right = node;
            left = left.next.as_mut().unwrap();
        }

        left.next = left.next.as_mut().unwrap().next.clone();

        dummy.next
    }
}