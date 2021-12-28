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

pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    

    // while let (Some(one), Some(two)) = (&list1, &list2) {
    //     let next_one_node = Box::new(ListNode::new(one.val));
    //     let next_two_node = Box::new(ListNode::new(two.val));

    //     if two.next.is_some() {
    //         next_two_node.next = two.next;
    //     }
        
    // }

    list1
}
