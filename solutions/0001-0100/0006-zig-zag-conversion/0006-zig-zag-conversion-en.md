---
title: "0006 Zigzag Conversion - EN"
problemUrl: "https://leetcode.com/problems/zigzag-conversion/"
difficulty: "Medium"
pubDate: "2026-01-14"
tags: ["string", "math", "optimization"]
complexity:
  time: "O(n)"
  space: "O(1)"
---

# Zigzag Conversion: From Visual Simulation to Mathematical Abstraction

## The Problem
The problem asks us to take a string and reorder it as if it were written in a zigzag pattern given a specific number of rows, and then read it line by line.

It seems like a trivial character manipulation problem, but it hides a valuable lesson on how a visual approach can betray us in terms of performance.

## The Intuition Trap: Linked Lists
When I first saw this problem, my mind went straight to physical simulation. I imagined $N$ rows forming dynamically while a "dealer" distributed the letters from top to bottom and then diagonally upwards.

The "perfect" data structure for this? **Linked Lists**.
Visually they were intuitive: I could create `numRows` list heads and hook nodes as I traversed the original string. In the end, I would just have to concatenate the lists.

My first approach was visually appealing, but **rude to the compiler and processor**:

```c
#include <stdlib.h>

typedef struct Node {
    char value;
    struct Node* next;
} Node; 

Node** create_tableu(int num_rows) {
    Node** tableu = (Node**)malloc(sizeof(Node*) * num_rows);
    for(int i = 0; i < num_rows; i++){
        Node* temp = (Node*) malloc(sizeof(Node));
        temp->value = '\0';
        temp->next = NULL;
        tableu[i] = temp;
    }
    return tableu;
}

Node* add_char(Node* row, char c){
    Node* temp = (Node*)malloc(sizeof(Node));
    temp->value = '\0';
    temp->next = NULL;

    Node* aux = row;
    aux->value = c;
    aux->next = temp;
    return temp;
}

char* convert(char* s, int numRows) {
    if(numRows == 1) return s;

    // LITERAL SIMULATION (Naive)
    // Use linked lists to represent each row.
    // PROBLEM: One malloc per EACH character of the input.
    // This fragments memory and the syscall overhead is massive.
    
    int currentRow = 0, i = 0, step = 1;
    char currentChar = s[i];
    Node** tableu = create_tableu(numRows);
    Node** tails = (Node**) malloc(sizeof(Node*) * numRows);
    for (int idx = 0; idx < numRows; idx++) {
        tails[idx] = tableu[idx];
    }
    
    while(currentChar != '\0') {
         // EXPENSIVE! A dynamic allocation in the hot path.
        tails[currentRow] = add_char(tails[currentRow], currentChar);
        
        // Zigzag "bounce" logic
        if (step > 0) {
            step = (currentRow + step < numRows) ? 1 : -1;
        } else {
            step = (currentRow + step < 0) ? 1 : -1;
        }

        currentRow += step;
        currentChar = s[++i];
    }

    // Reconstruction of the final string
    char* result = (char*)malloc(sizeof(char) * i + 1);
    currentRow = 0; 
    int j = 0;
    for (int k = 0; k < numRows; k++){
        Node* aux = tableu[k];
        while (aux->next && aux->value != '\0') {
            result[j] = aux->value;
            aux = aux->next;
            if (j < i) j++;
        }
    }
    result[i] = '\0';
    
    // NOTE: We would need to free all node memory here,
    // which would make the code even slower.
    return result;   
}
```

### The Reality Check

My intuition collapsed when I saw the results. Although the logic was correct, the *runtime* was tens of milliseconds.

I had fallen into a rookie mistake: **assuming that using pointers automatically means speed**.
I was performing a `malloc` call for every character of the string. Dynamic memory allocation (Syscalls) is expensive. I was fragmenting memory and destroying cache locality. I wanted to reach 0ms, but with that approach, it was impossible.

## The Paradigm Shift: Math over Memory

I went back to the drawing board. I decided to stop "moving" the data and start calculating where it should be.

Observing the indices, I noticed that the jumps followed a precise mathematical pattern:

1. **Top and Bottom Rows:** Have a constant jump (`jump = 2 * numRows - 2`).
2. **Intermediate Rows:** Have the same main jump, but with an "extra step" in between (the diagonal letter).

The formula for that intermediate index was the hardest to deduce: `i + jump - 2 * r`.

### The Final Optimization: Stealing Nanoseconds

With the mathematical logic, I achieved a decent time (3ms), but I knew I could squeeze out more. I reviewed my code and noticed redundancies: repeated or implicit `strlen` calls.

In C, every cycle counts. This final version calculates the length only once, makes a single exact memory allocation, and uses pure arithmetic to fill the buffer.

```c
#include <stdlib.h>
#include <string.h>

char* convert(char* s, int numRows) {
    if (numRows == 1) return s;
    
    int len = strlen(s);
    
    // A single exact memory allocation. 
    // Maximum efficiency and cache locality.
    char* result = (char*)malloc(len + 1);
    
    int idx_write = 0;
    
    // The zigzag "cycle" repeats every (2 * numRows - 2) characters.
    // Example with numRows=3:
    // P   A   H   N
    // A P L S I I G
    // Y   I   R
    // The jump is 2*3 - 2 = 4. From 'P' to 'A' (top) there are 4 positions.
    int jump = 2 * numRows - 2;

    for (int r = 0; r < numRows; r++) {
        // We iterate over the characters that fall "vertically" in row 'r'
        for (int i = r; i < len; i += jump) {
            result[idx_write++] = s[i];
            
            // For intermediate rows (not the first or the last),
            // there is an extra character in the "diagonal" between the vertical columns.
            if (r > 0 && r < numRows - 1) {
                // Pure math to find the diagonal position
                int diagonal_idx = i + jump - 2 * r;
                
                if (diagonal_idx < len) {
                    result[idx_write++] = s[diagonal_idx];
                }
            }
        }
    }

    result[len] = '\0';
    return result;
}
```

## Conclusion

I went from simulating physical movement (slow and expensive) to modeling mathematical behavior (fast and efficient).

This exercise reinforced a fundamental lesson of systems programming: **Mathematical abstraction almost always beats literal simulation.** Avoiding `malloc` inside loops and working with contiguous memory is the difference between code that works and high-performance code.
