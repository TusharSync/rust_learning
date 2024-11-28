// #include <stdio.h>
// #include <stdlib.h>
// #include <string.h>

// int main() {
//     int bufferSize = 1;

//     // Allocate memory for a string buffer and initialize to 0
//     char *buffer = (char *)calloc(bufferSize, sizeof(char));
//     if (buffer == NULL) {
//         printf("Memory allocation failed\n");
//         return 1; // Exit if allocation fails
//     }

//     // Copy a string into the buffer
//     strcpy(buffer, "Hello, sdddddddddddddddddddddddddddddWorld!");

//     // Display the string
//     printf("String in buffer: %s\n", buffer);

//     // Free allocated memory
//     free(buffer);
//     return 0;
// }
#include <stdio.h>
#include <time.h>

int my_async_function()
{
    for (int i = 1; i < 1000000000; ++i)
    { // Reduced to a smaller range to avoid too many iterations

        printf("Doing something async... %d\n", i);
    }
    return 42;
}

int main()
{
    // Start measuring time
    clock_t start = clock();

    // Run the function synchronously
    int result = my_async_function();
    printf("Result: %d\n", result);

    // Calculate elapsed time
    clock_t end = clock();
    double duration = (double)(end - start) / CLOCKS_PER_SEC;
    printf("Time taken: %.6f seconds\n", duration);

    return 0;
}
