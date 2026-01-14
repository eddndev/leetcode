---
title: "3454 Separate Squares II - EN"
problemUrl: "https://leetcode.com/problems/separate-squares-ii/"
difficulty: "Hard"
pubDate: "2026-01-14"
tags: ["segment-tree", "sweep-line", "binary-search", "coordinate-compression"]
complexity:
  time: "O(N log N)"
  space: "O(N)"
---

# Taming Geometry with Segment Trees in C

## The Initial Abyss
When I saw this problem, and although I didn't know it yet, I knew I had to use some complex data structure, something like a hash map or binary tree. Seeing what it actually consisted of overwhelmed me for a few moments, but I took a breath, calmed down, and started thinking. What is required to solve this class of problems?

Then I started to see the shape of it. The constraints were very strict: the shaded area could only be counted once (union of areas). I knew I had to have something that took into account the entire active horizontal area for certain vertical coordinates. But how did I build a data structure capable of doing such a thing?

I pressured myself a bit seeing the low acceptance rate the problem had. A bit of advice: don't look at what others have achieved, focus on what you know, and if you don't know, the first step is always to find the things you don't know. It seems too counterintuitive to me, that advice, but thanks to it I discovered a quite interesting structure: **Segment Trees**.

To be honest, I had never heard of them in my life, but they are practically binary trees that allow efficiently querying and updating information about intervals. There I could store the "coverage" of the X axis. And from the start I knew I couldn't create a matrix covering all points in X (since they go up to $10^9$), only the ones that mattered to us.

## The Strategy: "Sweep Line" + Coordinate Compression
The puzzle was completed when I understood that I didn't need to process the static 2D plane. I could use a **Sweep Line**: an imaginary line that rises from $Y=0$.

Every time the line touches the bottom edge of a square, we "activate" a range in the Segment Tree. When it touches the top edge, we "deactivate" it. The area is simply the integral of: `(Current Height - Previous Height) * Covered Width in X`.

### The C Challenge: Doing it by Hand
In high-level languages, there are libraries for ordered maps. In C, I had to implement all the "tooling" from scratch:
1.  **Coordinate Compression:** Since the X coordinates were huge, I mapped them to small indices ($0, 1, 2...$) using `qsort`, a manual `unique` algorithm, and binary search.
2.  **Array-Based Segment Tree:** I implemented the tree in a simple linear array, handling the recursion to calculate the covered length (`tree_len`) based on whether a node was fully covered (`count > 0`) or partially covered by its children.

## The "Old School" Optimization
The problem requires dividing the total area into two equal parts. This implies a "chicken and egg" problem: I need the Total Area to know what the half is, but I don't have the Total Area until I finish processing.

The naive solution would be:
1.  Run the algorithm to get the Total.
2.  `free` everything.
3.  `malloc` again and initialize everything to search for the half.

But in C, we can be cleverer. Instead of destroying and creating memory (expensive), I used `memset` to "reset" the tree to zeros instantly between the first and second pass.

```c
// ... (Segment Tree and Sweep Logic) ...

    // --- SECOND SWEEP ---
    // Instead of reallocating memory, we wipe the existing block.
    // This saves valuable system memory management cycles.
    memset(tree_count, 0, (4 * num_intervals + 1) * sizeof(int));
    memset(tree_len, 0, (4 * num_intervals + 1) * sizeof(double));

// ...

```

## Final Solution

This combination of **Coordinate Compression + Segment Tree + Sweep Line** resulted in an extremely efficient solution.

```c
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Event Structure: The heart of the Sweep Line.
// Represents the bottom (type=1) or top (type=-1) edge of a square.
typedef struct {
    int y;
    int type; // 1 = Square Enters, -1 = Square Leaves
    int x_start;
    int x_end;
} Event;

// Sort events from bottom to top (smallest Y).
// If ties in Y, process entries (+1) before exits (-1).
int compareEvent(const void* a, const void* b) {
    Event* eventA = (Event*) a;
    Event* eventB = (Event*) b;

    if (eventA->y != eventB->y) {
        return eventA->y - eventB->y;
    }

    return eventA->type - eventB->type;
}

int compareInts(const void* a, const void* b) {
    return (*(int*)a - *(int*)b);
}

// Binary search to find the mapped (discretized) index of an original X coordinate.
int get_x_index(int* x_coords, int n, int target) {
    int left = 0;
    int right = n - 1;
    while (left <= right) {
        int mid = left + (right - left) / 2;
        if (x_coords[mid] == target) return mid;
        if (x_coords[mid] < target) left = mid + 1;
        else right = mid - 1;
    }
    return -1;
}

// Segment Tree Logic.
// `node`: current index in the tree array.
// `start`, `end`: range of compressed indices covered by this node.
// `l`, `r`: range of the current query (the square entering/leaving).
void update(
    int node, int start, int end, int l, int r, int val,
    int* tree_count, double* tree_len, int* x_coords
) {
    if (l >= r || start >= r || end <= l) return;

    // If the node is fully contained in the update range, add/subtract `val`.
    if (l <= start && end <= r) tree_count[node] += val;
    else {
        // If partial, go down to children.
        int mid = start + (end - start) / 2;

        update(node * 2, start, mid, l, r,val, tree_count, tree_len, x_coords);
        update(node * 2 + 1, mid, end, l, r, val, tree_count, tree_len, x_coords);
    }

    // Recalculation of `tree_len[node]`: Active length covered by this node.
    // CASE 1: This node has a count > 0 -> Its entire range is covered.
    if (tree_count[node] > 0) tree_len[node] = (double)(x_coords[end] - x_coords[start]);
    // CASE 2: It is a leaf (and count == 0) -> Covers nothing.
    else if (end - start == 1) tree_len[node] = 0.0;
    // CASE 3: It is an internal node (and count == 0) -> Sum of what its children cover.
    else tree_len[node] = tree_len[node * 2] + tree_len[node * 2 +1];
}

double separateSquares(int** squares, int squaresSize, int* squaresColSize) {
    Event* events  = (Event*) malloc(sizeof(Event) * 2 * squaresSize);

    // Coordinate Compression: Map giant X coordinates to indices [0, 2N]
    int* x_coords = (int*)malloc(sizeof(int) * 2 * squaresSize);

    int e_idx = 0; // Index for events
    int x_idx = 0; // Index for x_coords

    for (int i = 0; i < squaresSize; i++ ) {
        int x = squares[i][0];
        int y = squares[i][1];
        int l = squares[i][2];
        
        // Entry event (bottom edge)
        events[e_idx].y = y;
        events[e_idx].type = 1;
        events[e_idx].x_start = x;
        events[e_idx].x_end = x + l;
        e_idx++;

        // Exit event (top edge)
        events[e_idx].y = y + l;
        events[e_idx].type = -1;
        events[e_idx].x_start = x;
        events[e_idx].x_end = x + l;
        e_idx++;

        x_coords[x_idx++] = x;
        x_coords[x_idx++] = x + l;
    }

    // Critical step: Sort events by Y, X coordinates by value.
    qsort(events, 2 * squaresSize, sizeof(Event), compareEvent);
    qsort(x_coords, 2 * squaresSize, sizeof(int), compareInts);

    // Unique: Remove duplicates in X to build base intervals.
    int unique_count = 0;
    if (2 * squaresSize > 0) {
        int write_idx = 0;

        for (int read_idx = 1; read_idx < 2 * squaresSize; read_idx++) {
            if (x_coords[read_idx] != x_coords[write_idx]) {
                write_idx++;
                x_coords[write_idx] = x_coords[read_idx]; // Copy
            }
        }
        unique_count = write_idx + 1;
    }

    int num_intervals = unique_count - 1;
    if (num_intervals <= 0) {
        free(events); free(x_coords);
        return 0.0;
    }

    // Segment Tree in flat array (4N is enough for worst case).
    int* tree_count = (int*)calloc(4 * num_intervals + 1, sizeof(int));
    double* tree_len = (double*)calloc(4 * num_intervals + 1, sizeof(double));

    double total_area = 0.0;

    // --- FIRST SWEEP: CALCULATE TOTAL AREA ---
    for (int i = 0; i < 2 * squaresSize; i++) {
        if (i > 0) {
            double dy = (double)(events[i].y - events[i-1].y);
            // Slice Area = Height (dy) * Covered Width (tree_len[1])
            total_area += dy * tree_len[1];
        }

        int idx_start = get_x_index(x_coords, unique_count, events[i].x_start);
        int idx_end = get_x_index(x_coords, unique_count, events[i].x_end);

        update(
            1, 0, num_intervals, idx_start, idx_end, events[i].type,
            tree_count, tree_len, x_coords
        );
    }

    // --- STRATEGIC RESET ---
    // Clean the tree with memset (very fast) to reuse it.
    memset(tree_count, 0, (4 * num_intervals + 1) * sizeof(int));
    memset(tree_len, 0, (4 * num_intervals + 1) * sizeof(double));

    double target = total_area / 2.0;
    double current_area = 0.0;
    double final_y = events[0].y;

    // --- SECOND SWEEP: FIND THE CUT LINE ---
    for (int i = 0; i < 2 * squaresSize; i++) {
        if (i > 0) {
            double dy = (double)(events[i].y - events[i - 1].y);
            double width = tree_len[1];
            double slice_area = dy * width;

            // Do we exceed half with this slice?
            if(current_area + slice_area >= target) {
                double missing_area = target - current_area;
                
                // Linear interpolation inside current slice to find exact Y.
                if (width > 0) final_y = events[i - 1].y + (missing_area / width);
                else final_y = events[i - 1].y;

                goto cleanup;
            }

            current_area += slice_area;
        }

        int idx_start = get_x_index(x_coords, unique_count, events[i].x_start);
        int idx_end = get_x_index(x_coords, unique_count, events[i].x_end);

        update(
            1, 0, num_intervals, idx_start, idx_end, events[i].type,
            tree_count, tree_len, x_coords
        );
    }

cleanup:
    free(events);
    free(x_coords);
    free(tree_count);
    free(tree_len);
    return final_y;
}
```

## Result

Although the final time was **267ms** (expected given the $O(N \log N)$ complexity and magnitude of the problem), the true victory was not the *runtime*.

What I value most is the satisfaction of having understood and applied a new data structure: the **Segment Tree**. Seeing how an abstraction can tame a chaotic geometric problem and turn it into a series of logical and discrete steps has been one of the best lessons of this challenge.
