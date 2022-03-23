#include <PreonEngine/PreonEngine.hpp>
#include <PreonEngine/Button.hpp>

#include <iostream>

int main() {
    preon::Page home("Home");
    preon::ButtonSpawner::newButton(home, "To Options");


    preon::Page options("Options");
    preon::ButtonSpawner::newButton(options, "To Home");
    preon::ButtonSpawner::newButton(options, "To Home");
    preon::ButtonSpawner::newButton(options, "To Home");


    std::cout << options.find(preon::Button::typeID()).size() << std::endl;

    return 0;
}