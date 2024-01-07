#include <cstdlib>
#include <string>
#include <iostream>
#include <fstream>

void startw() {
    #if defined(_WIN32) || defined(_WIN64)
        system("start app.exe");
    #elif defined(__linux__) || defined(__APPLE__) || defined(__MACH__)
        system("./app");
    #else
        #error "Unsupported operating system."
    #endif
}

bool fileExists(const std::string& filename) {
    std::ifstream file(filename);
    return file.good();
}

void start() {
    if (fileExists("app")) {
        #if defined(_WIN32) || defined(_WIN64)
            system("app.exe");
        #elif defined(__linux__) || defined(__APPLE__) || defined(__MACH__)
            system("./app");
        #else
            #error "Unsupported operating system."
        #endif
    }
}

void help() {
    std::cout << "\x1b[0;36m1. \x1b[0;32mStart terminal in new window" << std::endl;
    std::cout << "\x1b[0;36m2. \x1b[0;32mStart terminal in current window" << std::endl;
    std::cout << "\x1b[0;36m3. \x1b[0;32mExit\x1b[0m\n" << std::endl;
}