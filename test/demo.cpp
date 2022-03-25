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
    }
};

int main() {
    Page home("Home");
    ButtonSpawner::newButton(home, "To Options");


    Page options("Options");
    ButtonSpawner::newButton(options, "To Home");
    ButtonSpawner::newButton(options, "To Home");
    ButtonSpawner::newButton(options, "To Home");

    std::cout << "Creating UI" << std::endl;

    for (int i = 0; i < 1000000; i++) {
        Entity helloPrinter;
        helloPrinter.addComponent(options.allocateComponent(new Printer("hello")));
        options.addEntity(helloPrinter);
    }

    Entity hiPrinter;
    hiPrinter.addComponent(options.allocateComponent(new Printer("hi")));
    options.addEntity(hiPrinter);

    options.addSystem(new PrinterSystem());

    // App app;
    // app.addPage(&home);
    // app.addPage(&options);
    // app.mainloop();

    std::cout << "Done!" << std::endl;

    int frame = 0;
    while(1) {
        std::cout << "Processing..." << std::endl;

        options.update();

        // sleep(1);

        std::cout << "Frame " << frame++ << " processed!" << std::endl;
    }

    return 0;
}