#include "Button.hpp"

#include "logging/logging.h"

using namespace preon;

Entity *ButtonSpawner::newButton(Page& page, std::string label) {
    debug("Creating button \"%s\"", label.c_str());

    Entity e;
    e.addComponent(page.allocateComponent(new Position(10, 50)));
    e.addComponent(page.allocateComponent(new Collider(Vector2(10, 50), Vector2(100, 50))));
    e.addComponent(page.allocateComponent(new Button(label)));
    page.addEntity(e);

    return page.lastEntity();
}