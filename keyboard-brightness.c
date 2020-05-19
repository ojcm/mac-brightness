/*
    Set Mac Keyboard Backlight Brightness
*/

enum {
  kGetSensorReadingID = 0,  // getSensorReading(int *, int *)
  kGetLEDBrightnessID = 1,  // getLEDBrightness(int, int *)
  kSetLEDBrightnessID = 2,  // setLEDBrightness(int, int, int *)
  kSetLEDFadeID = 3,        // setLEDFade(int, int, int, int *)
};

#include <mach/mach.h>
#include <IOKit/IOKitLib.h>
#include <CoreFoundation/CoreFoundation.h>
#include "keyboard-brightness/rust_keyboard_brightness.h"

io_connect_t getDataPort(void) {
    io_connect_t      dataPort;
    kern_return_t     kr;
    io_service_t      serviceObject;

    // Look up a registered IOService object whose class is AppleLMUController
    serviceObject = IOServiceGetMatchingService(kIOMasterPortDefault, IOServiceMatching("AppleLMUController"));

    if (!serviceObject) {
        printf("Failed to connect to AppleLMUController\n");
        return 0;
    }

    // Create a connection to the IOService object
    kr = IOServiceOpen(serviceObject, mach_task_self(), 0, &dataPort);
    IOObjectRelease(serviceObject);

    if (kr != KERN_SUCCESS) {
        printf("Failed to open IoService object\n");
        return 0;
    }
    return dataPort;
}

float getKeyboardBrightness(void) {
    float brightness;
    kern_return_t kr;

    uint64_t inputCount = 1;
    uint64_t inputValues[1] = {0};

    uint32_t outputCount = 1;
    uint64_t outputValues[1];

    uint32_t out_brightness;

    kr = IOConnectCallScalarMethod(
        getDataPort(),
        kGetLEDBrightnessID,
        inputValues,
        inputCount,
        outputValues,
        &outputCount
    );

    if (kr != KERN_SUCCESS) {
        printf("getKeyboardBrightness() error\n");
        return 0;
    }

    brightness = outputValues[0];
    brightness /= 0xfff;
    return (float)brightness;
}

void setKeyboardBrightness(float new_brightness) {
    kern_return_t kr;

    uint64_t inputCount = 2;
    uint64_t inputValues[2] = {
        0, // Unknown input
        new_brightness * 0xfff
    };

    uint32_t outputCount = 1;
    uint64_t outputValues[1];

    kr = IOConnectCallScalarMethod(
        getDataPort(),
        kSetLEDBrightnessID,
        inputValues,
        inputCount,
        outputValues,
        &outputCount
    );

    if (kr != KERN_SUCCESS) {
        printf("setKeyboardBrightness() error\n");
        return;
    }
}


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
