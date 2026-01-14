#include <stdlib.h>
#include <string.h>

char *convert(char *s, int numRows) {
    if (numRows == 1) return s;

    int len = strlen(s);

    char *result = (char *)malloc(len + 1);

    int idx_write = 0;
    int jump = 2 * numRows - 2;

    for (int r = 0; r < numRows; r++) {
        for (int i = r; i < len; i += jump) {
            result[idx_write++] = s[i];

            if (r > 0 && r < numRows - 1) {
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