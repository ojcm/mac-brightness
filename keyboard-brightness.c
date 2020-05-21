/*
    Set Mac Keyboard Backlight Brightness
*/

#include <stdio.h>
#include "keyboard-brightness/rust_keyboard_brightness.h"

int main(int argc, char **argv) {
    // ./kbrightness 0.523
    float brightness;
    if (argc > 1 && sscanf(argv[1], "%f", &brightness) == 1) {
        setKeyboardBrightness(brightness);
    } else {
        print_brightness(getKeyboardBrightness());
    }
    exit(0);
}
