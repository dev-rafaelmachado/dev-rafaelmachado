#include <iostream>
#include <vector>
#include <string>

class Developer {
public:
    std::string name = "Rafael";
    int age = 20;
    std::string location = "Curitiba, PR - Brazil";
    std::vector<std::string> languages = {
        "JavaScript", "TypeScript", "Rust", "Java", "C++", "Python"
    };
};

class Skills {
public:
    std::vector<std::string> skills = {
        "Frontend Development",
        "Backend Development",
        "Studying all the time"
    };
};

class Bio : public Developer, public Skills {
public:
    Bio() {
        std::cout << "Hello, my name is " << name << std::endl;
        std::cout << "I am " << age << " years old" << std::endl;
        std::cout << "I live in " << location << std::endl;
        std::cout << "I am a developer and I have the following skills:" << std::endl;
        for (auto skill : skills) {
            std::cout << " - " << skill << std::endl;
        }
        std::cout << "I am also knowledgeable in the following languages:" << std::endl;
        for (auto language : languages) {
            std::cout << " - " << language << std::endl;
        }
    }
};

int main() {
    Bio bio;
    return 0;
}