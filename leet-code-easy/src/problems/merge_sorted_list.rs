// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    match (list1, list2) {
        (Some(l1), None) => Some(l1),
        (None, Some(l2)) => Some(l2),
        (None, None) => None,
        (Some(l1), Some(l2)) => {
            if l1.val < l2.val {
                Some(Box::new(ListNode {
                    val: l1.val,
                    next: merge_two_lists(l1.next, Some(l2)),
                }))
            } else {
                Some(Box::new(ListNode {
                    val: l2.val,
                    next: merge_two_lists(Some(l1), l2.next),
                }))
            }
        }
    }
}

pub fn build_list_from_vec(my_vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut result = None;
    for &el in my_vec.iter().rev() {
        result = Some(Box::new(ListNode {
            val: el,
            next: result,
        }))
    }

    result
}

pub fn build_vec_from_list(list: &mut Option<Box<ListNode>>) -> Vec<i32> {
    let mut result = Vec::new();

    while let Some(l) = list {
        result.push(l.val);
        *list = l.next.take();
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn hey() {
        let list1 = build_list_from_vec(vec![1, 3, 4, 4]);
        let list2 = build_list_from_vec(vec![1, 2, 5]);

        let mut new_list = merge_two_lists(list1, list2);

        assert_eq!(
            build_vec_from_list(&mut new_list),
            vec![1, 1, 2, 3, 4, 4, 5]
        );
    }
}
