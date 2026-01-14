#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct {
    int y;
    int type;  // 1 Entry, -1 Out
    int x_start;
    int x_end;
} Event;

int compareEvent(const void *a, const void *b) {
    Event *eventA = (Event *)a;
    Event *eventB = (Event *)b;

    if (eventA->y != eventB->y) {
        return eventA->y - eventB->y;
    }

    return eventA->type - eventB->type;
}

int compareInts(const void *a, const void *b) {
    return (*(int *)a - *(int *)b);
}

int get_x_index(int *x_coords, int n, int target) {
    int left = 0;
    int right = n - 1;
    while (left <= right) {
        int mid = left + (right - left) / 2;
        if (x_coords[mid] == target) return mid;
        if (x_coords[mid] < target)
            left = mid + 1;
        else
            right = mid - 1;
    }
    return -1;
}

void update(int node, int start, int end, int l, int r, int val, int *tree_count, double *tree_len,
            int *x_coords) {
    if (l >= r || start >= r || end <= l) return;

    if (l <= start && end <= r)
        tree_count[node] += val;
    else {
        int mid = start + (end - start) / 2;

        update(node * 2, start, mid, l, r, val, tree_count, tree_len, x_coords);
        update(node * 2 + 1, mid, end, l, r, val, tree_count, tree_len, x_coords);
    }

    if (tree_count[node] > 0)
        tree_len[node] = (double)(x_coords[end] - x_coords[start]);
    else if (end - start == 1)
        tree_len[node] = 0.0;
    else
        tree_len[node] = tree_len[node * 2] + tree_len[node * 2 + 1];
}

double separateSquares(int **squares, int squaresSize, int *squaresColSize) {
    Event *events = (Event *)malloc(sizeof(Event) * 2 * squaresSize);

    // Allocate memory for coordinates X (2 for each square)
    int *x_coords = (int *)malloc(sizeof(int) * 2 * squaresSize);

    int e_idx = 0;  // Index for events
    int x_idx = 0;  // Index for x_coords

    for (int i = 0; i < squaresSize; i++) {
        int x = squares[i][0];
        int y = squares[i][1];
        int l = squares[i][2];

        events[e_idx].y = y;
        events[e_idx].type = 1;
        events[e_idx].x_start = x;
        events[e_idx].x_end = x + l;
        e_idx++;

        events[e_idx].y = y + l;
        events[e_idx].type = -1;
        events[e_idx].x_start = x;
        events[e_idx].x_end = x + l;
        e_idx++;

        x_coords[x_idx++] = x;
        x_coords[x_idx++] = x + l;
    }

    qsort(events, 2 * squaresSize, sizeof(Event), compareEvent);
    qsort(x_coords, 2 * squaresSize, sizeof(int), compareInts);

    int unique_count = 0;
    if (2 * squaresSize > 0) {
        int write_idx = 0;

        for (int read_idx = 1; read_idx < 2 * squaresSize; read_idx++) {
            if (x_coords[read_idx] != x_coords[write_idx]) {
                write_idx++;
                x_coords[write_idx] = x_coords[read_idx];  // Copy
            }
        }
        unique_count = write_idx + 1;
    }

    int num_intervals = unique_count - 1;
    if (num_intervals <= 0) {
        free(events);
        free(x_coords);
        return 0.0;
    }

    int *tree_count = (int *)calloc(4 * num_intervals + 1, sizeof(int));
    double *tree_len = (double *)calloc(4 * num_intervals + 1, sizeof(double));

    double total_area = 0.0;

    for (int i = 0; i < 2 * squaresSize; i++) {
        if (i > 0) {
            double dy = (double)(events[i].y - events[i - 1].y);
            total_area += dy * tree_len[1];
        }

        int idx_start = get_x_index(x_coords, unique_count, events[i].x_start);
        int idx_end = get_x_index(x_coords, unique_count, events[i].x_end);

        update(1, 0, num_intervals, idx_start, idx_end, events[i].type, tree_count, tree_len,
               x_coords);
    }

    memset(tree_count, 0, (4 * num_intervals + 1) * sizeof(int));
    memset(tree_len, 0, (4 * num_intervals + 1) * sizeof(double));

    double target = total_area / 2.0;
    double current_area = 0.0;
    double final_y = events[0].y;

    for (int i = 0; i < 2 * squaresSize; i++) {
        if (i > 0) {
            double dy = (double)(events[i].y - events[i - 1].y);
            double width = tree_len[1];
            double slice_area = dy * width;

            if (current_area + slice_area >= target) {
                double missing_area = target - current_area;

                if (width > 0)
                    final_y = events[i - 1].y + (missing_area / width);
                else
                    final_y = events[i - 1].y;

                goto cleanup;
            }

            current_area += slice_area;
        }

        int idx_start = get_x_index(x_coords, unique_count, events[i].x_start);
        int idx_end = get_x_index(x_coords, unique_count, events[i].x_end);

        update(1, 0, num_intervals, idx_start, idx_end, events[i].type, tree_count, tree_len,
               x_coords);
    }

cleanup:
    free(events);
    free(x_coords);
    free(tree_count);
    free(tree_len);
    return final_y;
}