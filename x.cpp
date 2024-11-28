#include <iostream>
#include <vector>    // Include the header for std::vector
#include <algorithm> // Include the header for std::vector

int main()
{
    // Create a vector of integers
    std::vector<int> numbers = {};

    std::vector<int>::iterator maxElement = std::max_element(numbers.begin(), numbers.end());
    std::cout << "maximum element: " << *maxElement.base() << std::endl;
    return 0;
}
