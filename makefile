buildw:
	g++ -Wall -o launcher.exe launcher.cpp

buildl:
	g++ -o launcher launcher.cpp

run:
	g++ -Wall -o launcher.exe launcher.cpp
	./launcher.exe