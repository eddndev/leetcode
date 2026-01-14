#include <limits.h>

static inline int isDigit(char c) {
    return (c >= '0' && c <= '9');
}

int myAtoi(char *s) {
    char *iterator = s;

    while ((*iterator) == ' ') {
        iterator++;
    }

    int sign = ((*iterator) == '-') ? -1 : (*iterator) == '+';
    if (sign == 0 && !isDigit(*iterator)) return 0;
    if (sign == 0)
        sign = 1;
    else
        iterator++;

    long long acumulator = 0;
    while (isDigit(*iterator)) {
        int digit = (*iterator) - '0';
        acumulator = acumulator * 10 + digit;

        if (sign == -1 && acumulator > (long long)INT_MAX + 1) return INT_MIN;
        if (sign == 1 && acumulator > INT_MAX) return INT_MAX;

        iterator++;
    }

    return acumulator * sign;
}
