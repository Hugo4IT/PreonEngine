#include "PreonEngine.hpp"
#include "logging/logging.h"

#include <cstdarg>

using namespace preon;

void Entity::addComponent(Component *component) {
    this->components.push_back(component);
}

std::vector<Component*> *Entity::getComponents() {
    return &this->components;
}

Page::Page(std::string title) {
    debug("Creating page \"%s\"", title.c_str());
    
    this->title = title;
}

void Page::addEntity(Entity entity) {
    this->entities.push_back(entity);
}

Entity *Page::lastEntity() {
    return &this->entities.back();
}

std::vector<int> Page::find(int typeID) {
    std::vector<int> indices;
    debug("Finding components with type ID %d in %lu entities", typeID, this->entities.size());
    for (unsigned long i = 0; i < this->entities.size(); i++) {
        Entity e = this->entities[i];
        debug("Found entity with %lu components", e.getComponents()->size());
        for (Component *c : *e.getComponents()) {
            debug("Found component with type ID: %d", c->getTypeID());
            if (c->getTypeID() == typeID) {
                indices.push_back(i);
            }
        }
    }

    return indices;
}

Page::~Page() {

}