#include <PreonEngine/PreonEngine.hpp>
#include <PreonEngine/Button.hpp>

#include <iostream>
#include <cstdarg>

// Sleep function (temporary)
#ifdef _WIN32
#include <Windows.h>
#else
#include <unistd.h>
#endif

using namespace preon;

struct Printer : public Component {
    GEN_COMPONENT(Printer)

public:
    std::string message;

    Printer(std::string message) : message(message) {}
};

class PrinterSystem : public System {
public:
    GEN_SYSTEM(PrinterSystem)

    PrinterSystem(){}

    inline std::vector<int> query() {
        return std::vector<int>(1, Printer::typeID());
    }

    inline void system(std::vector<Component*> components) {
        Printer *ph = (Printer*)components[0];
        std::cout << ph->message << std::endl;
    }
};

int main() {
    Page home("Home");
    ButtonSpawner::newButton(home, "To Options");


    Page options("Options");
    ButtonSpawner::newButton(options, "To Home");
    ButtonSpawner::newButton(options, "To Home");
    ButtonSpawner::newButton(options, "To Home");

    Entity helloPrinter;
    helloPrinter.addComponent(options.allocateComponent(new Printer("hello")));
    options.addEntity(helloPrinter);

    Entity hiPrinter;
    hiPrinter.addComponent(options.allocateComponent(new Printer("hi")));
    options.addEntity(hiPrinter);

    options.addSystem(new PrinterSystem());

    App app;
    app.addPage(&home);
    app.addPage(&options);
    app.mainloop();

    return 0;
}