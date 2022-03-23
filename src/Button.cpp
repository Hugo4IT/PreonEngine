#include "Button.hpp"

#include "logging/logging.h"

using namespace preon;

Entity *ButtonSpawner::newButton(Page& destination, std::string label) {
    debug("Creating button \"%s\"", label.c_str());

    Entity e;
    e.addComponent(new Position(10, 50));
    e.addComponent(new Collider(Vector2(10, 50), Vector2(100, 50)));
    e.addComponent(new Button(label));
    destination.addEntity(e);
    
    return destination.lastEntity();
}