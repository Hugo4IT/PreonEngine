#pragma once

#include "PreonEngine.hpp"

#include <string>

namespace preon {
    struct Button : public Component {
        GEN_COMPONENT(Button)

    private:
        std::string label;

    public:
        Button(std::string label) : label(label) {}
    };

    class ButtonSpawner {
    public:
        static Entity *newButton(Page& destination, std::string label);
    };
}