#include <stdio.h>
#include <stdlib.h>

// Function to create a deep copy
int** deep_copy(int** original, int rows, int cols) {
    int** copy = (int**)malloc(rows * sizeof(int*));
    for (int i = 0; i < rows; i++) {
        copy[i] = (int*)malloc(cols * sizeof(int));  // Allocating new memory for inner arrays
        for (int j = 0; j < cols; j++) {
            copy[i][j] = original[i][j];  // Copying each element (deep copy)
        }
    }
    return copy;
}

int main() {
    // Original 2D array
    int rows = 2, cols = 3;
    int** original = (int**)malloc(rows * sizeof(int*));
    for (int i = 0; i < rows; i++) {
        original[i] = (int*)malloc(cols * sizeof(int));
    }

    // Initializing the original array
    original[0][0] = 1; original[0][1] = 2; original[0][2] = 3;
    original[1][0] = 4; original[1][1] = 5; original[1][2] = 6;

    // Deep copy
    int** deep_copy_array = deep_copy(original, rows, cols);

    // Modifying the deep copy
    deep_copy_array[0][0] = 99;

    // Printing the original and deep copy arrays
    printf("Original Array:\n");
    for (int i = 0; i < rows; i++) {
        for (int j = 0; j < cols; j++) {
            printf("%d ", original[i][j]);
        }
        printf("\n");
    }

    printf("\nDeep Copy Array:\n");
    for (int i = 0; i < rows; i++) {
        for (int j = 0; j < cols; j++) {
            printf("%d ", deep_copy_array[i][j]);
        }
        printf("\n");
    }

    // Freeing memory
    for (int i = 0; i < rows; i++) {
        free(original[i]);
        free(deep_copy_array[i]);  // Free each inner array
    }
    free(original);
    free(deep_copy_array);  // Free the outer array
    return 0;
}
