#include <stdio.h>
#include <stdlib.h>

// Function to create a shallow copy
int **shallow_copy(int **original, int rows)
{
    int **copy = (int **)malloc(rows * sizeof(int *));
    for (int i = 0; i < rows; i++)
    {
        // Only copying the pointer to the inner array (shallow copy)
        copy[i] = original[i];
    }
    return copy;
}

int main()
{
    // Original 2D array
    int rows = 2, cols = 3;
    int **original = (int **)malloc(rows * sizeof(int *));
    for (int i = 0; i < rows; i++)
    {
        original[i] = (int *)malloc(cols * sizeof(int));
    }

    // Initializing the original array
    original[0][0] = 1;
    original[0][1] = 2;
    original[0][2] = 3;
    original[1][0] = 4;
    original[1][1] = 5;
    original[1][2] = 6;

    // Shallow copy
    int **shallow_copy_array = shallow_copy(original, rows);

    // Modifying the shallow copy
    shallow_copy_array[0][0] = 99;

    // Printing the original and shallow copy arrays
    printf("Original Array:\n");
    for (int i = 0; i < rows; i++)
    {
        for (int j = 0; j < cols; j++)
        {
            printf("%d ", original[i][j]);
        }
        printf("\n");
    }

    printf("\nShallow Copy Array:\n");
    for (int i = 0; i < rows; i++)
    {
        for (int j = 0; j < cols; j++)
        {
            printf("%d ", shallow_copy_array[i][j]);
        }
        printf("\n");
    }

    // Freeing memory
    for (int i = 0; i < rows; i++)
    {
        free(original[i]);
    }
    free(original);
    free(shallow_copy_array); // Only freeing outer array pointers
    return 0;
}
