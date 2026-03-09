---
title: "1877 Minimize Maximum Pair Sum in Array - EN"
problemUrl: "https://leetcode.com/problems/minimize-maximum-pair-sum-in-array/"
difficulty: "Medium"
pubDate: "2026-03-08"
tags: ["array", "two-pointers", "greedy", "sorting"]
complexity:
  time: "O(n log n)"
  space: "O(1)"
---

# Minimize Maximum Pair Sum in Array: Pairing the Extremes

## The Problem
Given an array `nums` of even length, we need to pair it into `n / 2` pairs. We want to minimize the **maximum pair sum** across all pairs. The pair sum is simply the sum of the two elements in a pair. Return the minimum possible value of the maximum pair sum.

A naive approach would explore every possible way to pair the elements, but that grows factorially and is completely impractical. There is a greedy observation that solves it directly.

## The Intuition: Balancing the Extremes

The key insight is that if we want to minimize the maximum sum, we need to prevent two large numbers from ending up in the same pair. The best way to achieve this is to pair the largest number with the smallest, the second largest with the second smallest, and so on.

Think about why this works. If we sort the array, the largest element `nums[n-1]` has to be in some pair. If we pair it with anything other than the smallest element, we are wasting the small element: it will get paired with another element that could have absorbed some of the weight from the largest. By pairing `nums[0]` with `nums[n-1]`, we use the smallest element to offset the largest, distributing the load as evenly as possible.

This reasoning applies recursively: once `nums[0]` and `nums[n-1]` are paired, the same argument holds for `nums[1]` and `nums[n-2]`, and so on. The result is that the maximum pair sum is minimized when we pair symmetrically from the extremes toward the center.

## The Algorithm
1. **Sort** the array in ascending order.
2. **Use two pointers**: one at the start (`i`) and one at the end (`j`) of the array.
3. **At each step**, compute the pair sum `nums[i] + nums[j]` and update the maximum if this sum exceeds the current maximum.
4. **Advance** `i` forward and `j` backward until they cross.

## C Solution

The implementation includes a manual quicksort since the C standard library does not offer a sorting function with a straightforward interface for integer arrays. Once the array is sorted, the two-pointer loop walks from the extremes and tracks the maximum pair sum.

```c
#include <stdio.h>

void swap(int* a, int* b) {
    int temp = *a;
    *a = *b;
    *b = temp;
}

int partition(int arr[], int low, int high) {
    int pivot = arr[high];
    int i = low - 1;

    for (int j = low; j <= high - 1; j++) {
        if (arr[j] <= pivot) {
            i++;
            swap(&arr[i], &arr[j]);
        }
    }
    swap(&arr[i+1], &arr[high]);
    return i + 1;
}

void quicksort(int arr[], int low, int high) {
    if (low < high) {
        int pi = partition(arr, low, high);
        quicksort(arr, low, pi - 1);
        quicksort(arr, pi + 1, high);
    }
}

int minPairSum(int* nums, int numsSize){
    quicksort(nums, 0, numsSize - 1);

    int max_sum = 0;

    int i = 0;
    int j = numsSize - 1;

    while(i < j) {
        int current_sum = nums[i] + nums[j];

        if (current_sum > max_sum) max_sum = current_sum;

        i++;
        j--;
    }

    return max_sum;
}
```

## Conclusion

The time complexity is $O(n \log n)$ dominated by the sort, since the two-pointer traversal is linear. The space is $O(1)$ if we exclude the quicksort recursion stack (or $O(\log n)$ if we count it). What makes this problem satisfying is how the greedy intuition -- pair the largest with the smallest -- justifies itself so naturally: any other pairing can only worsen the maximum sum, never improve it. It is a classic example of how sorting transforms a combinatorial optimization problem into a simple linear scan.
