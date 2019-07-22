#include <stdlib.h>

#include "pvm.h"

int main(int argc, char* argv[]) {
    uint8_t data[32];
    size_t size;
    pvm_intf(&data[0], 32, &size);

    pvm_debug((char *)data);
    return 0;
}
