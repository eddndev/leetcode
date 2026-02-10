#include <stdbool.h>
#include <string.h>

int numMagicSquaresInside(int **grid, int gridSize, int *gridColSize) {
    size_t r = gridSize, c = gridColSize[0];

    if (r < 3 || c < 3) return 0;

    int result = 0;
    for (int i = 0; i <= r - 3; i++) {
        for (int j = 0; j <= c - 3; j++) {
            if (grid[i + 1][j + 1] != 5) {
                continue;
            }

            bool seen[10] = {false};
            bool valid = true;

            for (int k = i; k < i + 3; k++) {
                for (int m = j; m < j + 3; m++) {
                    int val = grid[k][m];
                    if (val < 1 || val > 9 || seen[val]) {
                        valid = false;
                        break;
                    }
                    seen[val] = true;
                }
                if (!valid) break;
            }

            if (!valid) continue;

            if (grid[i][j] + grid[i][j + 1] + grid[i][j + 2] != 15) continue;
            if (grid[i + 1][j] + grid[i + 1][j + 1] + grid[i + 1][j + 2] != 15) continue;
            if (grid[i + 2][j] + grid[i + 2][j + 1] + grid[i + 2][j + 2] != 15) continue;

            if (grid[i][j] + grid[i + 1][j] + grid[i + 2][j] != 15) continue;
            if (grid[i][j + 1] + grid[i + 1][j + 1] + grid[i + 2][j + 1] != 15) continue;
            if (grid[i][j + 2] + grid[i + 1][j + 2] + grid[i + 2][j + 2] != 15) continue;

            if (grid[i][j] + grid[i + 1][j + 1] + grid[i + 2][j + 2] != 15) continue;
            if (grid[i][j + 2] + grid[i + 1][j + 1] + grid[i + 2][j] != 15) continue;

            result++;
        }
    }
    return result;
}