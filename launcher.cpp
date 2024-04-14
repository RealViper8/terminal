#include "include/launch.h"
#include <iostream>
#include <fstream>
#include <stdio.h>

int main() {
    std::cout << "\n\x1b[1;36m--- \x1b[0;32mTerminal Menu\x1b[0;36m ---\x1b[0m\n" << std::endl;
    help();
    while (true) {
        std::string input = "";
        std::cout << "\x1b[1;36m>>>\x1b[0;32m ";
        
        getline(std::cin, input);
        std::cout << std::endl;

        if (input == "1" || input == "startwindow") {
            startw();    
        } else if (input == "2" || input == "start") {
            start();
        } else if (input == "3" || input == "exit") {
            printf("\x1b[0m");
            break;
        } else if (input == "help") {
            help();
        } else if (input == "clear" || input == "cls") {
            #if defined(_WIN32) || defined(_WIN64)
                    system("cls");
            #elif defined(__linux__) || defined(__APPLE__) || defined(__MACH__)
                    system("clear");
            #endif
        } else if (input == "write") {
            std::string write = "";
            std::cout << "\x1b[1;36mWich file do you want to write ?\n>>>\x1b[0;32m ";
            getline(std::cin, write);
            std::cout << std::endl;
            printf("\x1b[0m");

            std::string content = "";
            std::cout << "\x1b[1;36mContent ?\n>>>\x1b[0;32m";
            getline(std::cin, write);
            std::cout << std::endl;
            printf("\x1b[0m");

            std::ofstream file(write);
            if (file.is_open()) {
                file << content;
                file.close();
                std::cout << "\x1b[0;32mFile '" << write << "' wrote successfully!\x1b[0m" << std::endl;
            } else {
                std::cout << "\x1b[0;31mError opening file!\x1b[0m" << std::endl;
            }
        } else {
            std::cout << "\x1b[0;31mInvalid input type help for more info !\x1b[0m" << std::endl;
        }
        

        printf("\x1b[0m");
        std::cin.clear();
    }
    
    return EXIT_SUCCESS;
}