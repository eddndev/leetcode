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
