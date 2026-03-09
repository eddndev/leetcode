---
title: "0002 Add Two Numbers - EN"
problemUrl: "https://leetcode.com/problems/add-two-numbers/"
difficulty: "Medium"
pubDate: "2026-03-08"
tags: ["linked-list", "math", "recursion"]
complexity:
  time: "O(max(n, m))"
  space: "O(max(n, m))"
---

# Add Two Numbers: Elementary Arithmetic with Linked Lists

## The Problem
We are given two non-empty linked lists representing two non-negative integers. The digits are stored in **reverse order**, and each node contains a single digit. We must add the two numbers and return the result as a linked list.

For example, if we have `l1 = [2, 4, 3]` and `l2 = [5, 6, 4]`, they represent the numbers 342 and 465 respectively. The sum is 807, so the result would be `[7, 0, 8]`.

It looks straightforward, but there is a subtlety that makes it interesting: the **carry**.

## The Intuition: Adding Like in School
When I saw this problem, the first thing that came to mind was the column addition we learned as children. You write the two numbers one below the other, add digit by digit from right to left, and if the sum exceeds 9, you "carry the one."

And it turns out the problem already gives us the digits in the perfect order for this: **right to left** (least significant digit first). We don't need to reverse anything. We just traverse both lists simultaneously, add the corresponding values along with the carry, and build the result list node by node.

## The Key: Don't Forget the Final Carry
The most common mistake in this problem is forgetting that after traversing both lists, there might be a pending carry. If we add `[9, 9]` and `[1]`, we get `[0, 0, 1]` (99 + 1 = 100). If we don't check the carry at the end, we would lose that last digit.

The loop condition must cover three cases: remaining nodes in `l1`, remaining nodes in `l2`, or a non-zero carry. As long as any of these conditions is true, we keep iterating.

## The Algorithm
1. Create a dummy node as the head of the result list.
2. Initialize the carry to 0.
3. Traverse both lists while either has nodes or the carry is non-zero:
   - Add the values of the current nodes (if they exist) plus the carry.
   - The new digit is `sum % 10`.
   - The new carry is `sum / 10`.
   - Create a new node with the digit and link it.
4. Return `dummy.next`.

### Implementation in C
In C, we use a stack-allocated dummy node to avoid treating the first node as a special case. Each new digit is created with `malloc`.

```c
/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     struct ListNode *next;
 * };
 */
#include <stdlib.h>

struct ListNode* addTwoNumbers(struct ListNode* l1, struct ListNode* l2) {
    struct ListNode dummy;
    dummy.val = 0;
    dummy.next = NULL;

    struct ListNode* current = &dummy;
    int carry = 0;

    while (l1 != NULL || l2 != NULL || carry) {
        int sum = carry;

        if (l1 != NULL) {
            sum += l1->val;
            l1 = l1->next;
        }

        if (l2 != NULL) {
            sum += l2->val;
            l2 = l2->next;
        }

        carry = sum / 10;

        struct ListNode* newNode = (struct ListNode*)malloc(sizeof(struct ListNode));
        newNode->val = sum % 10;
        newNode->next = NULL;

        current->next = newNode;
        current = newNode;
    }

    return dummy.next;
}
```

### Implementation in Rust
In Rust, handling linked lists with `Option<Box<ListNode>>` is more verbose, but the pattern is the same. We use immutable references to traverse the input lists and build the result list by mutating the `current` pointer.

```rust
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
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        let mut current = &mut dummy;
        let mut carry = 0;
        let mut p1 = &l1;
        let mut p2 = &l2;

        while p1.is_some() || p2.is_some() || carry != 0 {
            let mut sum = carry;

            if let Some(node) = p1 {
                sum += node.val;
                p1 = &node.next;
            }

            if let Some(node) = p2 {
                sum += node.val;
                p2 = &node.next;
            }

            carry = sum / 10;
            current.next = Some(Box::new(ListNode::new(sum % 10)));
            current = current.next.as_mut().unwrap();
        }

        dummy.next
    }
}
```

## Conclusion
This problem is an excellent exercise for practicing simultaneous traversal of linked lists and carry management. The key isn't a sophisticated data structure or a hidden algorithmic trick, but rather attention to detail: correctly handling lists of different lengths and not forgetting the final carry. It is one of those problems where elegance lies in simplicity.
