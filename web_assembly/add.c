// add.c
#include <emscripten/emscripten.h>

// This annotation tells Emscripten to export this function.
EMSCRIPTEN_KEEPALIVE
int add(int a, int b) {
    return a + b;
}
