/**
 * Note: The returned array must be malloced, assume caller calls free().
 */
#include <stdlib.h>
#define HASH_SIZE 20003

int hash(int key) {
    return abs(key) % HASH_SIZE;
}

typedef struct Node {
    int key;
    int index;
    struct Node *next;
} Node;

typedef struct {
    Node *buckets[HASH_SIZE];
} HashTable;

// Insert a tuple { value, index}
void insert(HashTable *table, int key, int index) {
    int h = hash(key);
    Node *newNode = (Node *)malloc(sizeof(Node));
    newNode->key = key;
    newNode->index = index;

    // Insert in the beginning of the linked list
    newNode->next = table->buckets[h];
    table->buckets[h] = newNode;
}

int search(HashTable *table, int key) {
    int h = hash(key);
    Node *c = table->buckets[h];
    while (c != NULL) {
        if (c->key == key) {
            return c->index;
        }
        c = c->next;
    }
    return -1;
}

void freeTable(HashTable *table) {
    for (int i = 0; i < HASH_SIZE; i++) {
        Node *current = table->buckets[i];
        while (current != NULL) {
            Node *temp = current;
            current = current->next;
            free(temp);
        }
        table->buckets[i] = NULL;
    }
}

int *twoSum(int *nums, int numsSize, int target, int *returnSize) {
    HashTable table;
    for (int i = 0; i < HASH_SIZE; i++) table.buckets[i] = NULL;

    for (int i = 0; i < numsSize; i++) {
        int complement = target - nums[i];
        int foundIndex = search(&table, complement);

        if (foundIndex != -1 && i != foundIndex) {
            int *sol = (int *)malloc(2 * sizeof(int));

            sol[0] = foundIndex;
            sol[1] = i;
            *returnSize = 2;

            freeTable(&table);
            return sol;
        }
        insert(&table, nums[i], i);
    }

    *returnSize = 0;
    freeTable(&table);
    return NULL;
}