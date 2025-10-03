#include <emscripten/emscripten.h>

#include <stdio.h>

typedef struct {
    int count;
} Item;

EMSCRIPTEN_KEEPALIVE
// Function to sum the 'count' values in an array of 'Item' structures
int sumCounts(Item* array, int size) {
    int sum = 0;
    for (int i = 0; i < size; i++) {
        sum += array[i].count;  // Direct summing without additional checks
    }
    return sum;
}
