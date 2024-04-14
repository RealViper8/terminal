#include <string>
#include <cstdlib>
#include <iostream>
#include <fstream>

#ifndef LAUNCHER_H
#define LAUNCHER_H

bool fileExists(const std::string& filename) {
    std::ifstream file(filename);
    return file.good();
}

void startw() {
    if (fileExists("app") || fileExists("app.exe")) {
        #if defined(_WIN32) || defined(_WIN64)
            system("start app.exe");
        #elif defined(__linux__) || defined(__APPLE__) || defined(__MACH__)
            system("./app");
        #else
            #error "Unsupported operating system."
        #endif
    } else if (fileExists("terminal") || fileExists("terminal.exe")) {
        #if defined(_WIN32) || defined(_WIN64)
            system("start terminal.exe");
        #elif defined(__linux__) || defined(__APPLE__) || defined(__MACH__)
            system("./terminal");
        #else
            #error "Unsupported operating system."
        #endif
    } else {
        printf("\x1b[1;31mError: \x1b[0;31mFailed to start terminal. Make sure you have all the files and folders in the same directory.\x1b[0m\n");
    }
}

void start() {
    if (fileExists("app") || fileExists("app.exe")) {
        #if defined(_WIN32) || defined(_WIN64)
            system("app.exe");
        #elif defined(__linux__) || defined(__APPLE__) || defined(__MACH__)
            system("./app");
        #else
            #error "Unsupported operating system."
        #endif
    } else if (fileExists("terminal") || fileExists("terminal.exe")) {
        #if defined(_WIN32) || defined(_WIN64)
            system("terminal.exe");
        #elif defined(__linux__) || defined(__APPLE__) || defined(__MACH__)
            system("./terminal");
        #else
            #error "Unsupported operating system."
        #endif
    } else {
        printf("\x1b[1;31mError: \x1b[0;31mFailed to start terminal. Make sure you have all the files and folders in the same directory.\x1b[0m\n");
    }
}

void help() {
    std::cout << "\x1b[1;36mMenu: \x1b[0m" << std::endl;
    std::cout << "\x1b[0;36m1. \x1b[0;32mStart terminal in new window" << std::endl;
    std::cout << "\x1b[0;36m2. \x1b[0;32mStart terminal in current window" << std::endl;
    std::cout << "\x1b[0;36m3. \x1b[0;32mExit\x1b[0m\n" << std::endl;
    std::cout << "\x1b[1;36mCommnads: \x1b[0m" << std::endl;
    std::cout << "\x1b[0;36mclear \x1b[0;32mClears the screen\x1b[0m\n";
    std::cout << "\x1b[0;36mwrite \x1b[0;32mWrite to file\x1b[0m\n" << std::endl;
}

#endif