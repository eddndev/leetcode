#include <stdbool.h>
#include <string.h>

bool isMatch(char *s, char *p) {
    int m = strlen(s);
    int n = strlen(p);

    // DP Table
    bool dp[m + 1][n + 1];

    for (int i = 0; i <= m; i++) {
        for (int j = 0; j <= n; j++) {
            dp[i][j] = false;
        }
    }

    // Initial State
    // Empty String VS Empty pattern is TRUE
    dp[0][0] = true;

    for (int j = 1; j <= n; j++) {
        if (p[j - 1] == '*') {
            dp[0][j] = dp[0][j - 2];
        }
    }

    for (int i = 1; i <= m; i++) {
        for (int j = 1; j <= n; j++) {
            if (p[j - 1] == '.' || p[j - 1] == s[i - 1]) {
                dp[i][j] = dp[i - 1][j - 1];
            }

            else if (p[j - 1] == '*') {
                bool zero_match = dp[i][j - 2];

                bool char_match = (p[j - 2] == s[i - 1] || p[j - 2] == '.');

                bool one_plus_match = char_match && dp[i - 1][j];

                dp[i][j] = zero_match || one_plus_match;
            }

            else {
                dp[i][j] = false;
            }
        }
    }

    return dp[m][n];
}