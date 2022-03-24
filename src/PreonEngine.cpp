#include "PreonEngine.hpp"
#include "logging/logging.h"

#include <cstdarg>

using namespace preon;

void Entity::addComponent(int cIndex) {
    this->components.push_back(cIndex);
}

std::vector<int> Entity::getComponents() {
    return this->components;
}

Page::Page(std::string title) {
    debug("Creating page \"%s\"", title.c_str());
    
    this->title = title;
    this->ecsUpdates = 0;
}

void Page::addEntity(Entity entity) {
    this->entities.push_back(entity);
    this->ecsUpdates++;
}

int Page::allocateComponent(Component *component) {
    int i = this->components.size();
    this->components.push_back(component);
    this->ecsUpdates++;

    return i;
}

void Page::addSystem(System *system) {
    this->systems.push_back(system);
    this->ecsUpdates++;
}

Entity *Page::lastEntity() {
    return &this->entities.back();
}

void Page::update() {
    debug("Detected %d change(s)", this->ecsUpdates);
    if (this->ecsUpdates) {
        this->systemCache.clear();

        // First pass: Create list of components mapping to systems:
        // list(list(int)) - Indexed by Component::typeID();
        //      list(int)  - A list of systems that request that component
        //           int   - The System::typeID() of that system
        std::vector< std::vector<int> > systemsRequestingComponents;
        for (unsigned long i = 0; i < this->systems.size(); i++) {
            System *s = this->systems[i];
            
            std::vector<int> requestedComponents = s->query();

            int highestIndex = 0;
            for (int r : requestedComponents)
                if (r > highestIndex)
                    highestIndex = r;

            if (systemsRequestingComponents.size() <= highestIndex)
                systemsRequestingComponents.resize(highestIndex + 1);
            
            for (int r : requestedComponents) {
                systemsRequestingComponents[r].push_back(i);
            }
        }

        // Second pass: Create systemCache:
        // list(list(list(int))) - Indexed by System::typeID();
        //      list(list(int))  - List of componenents for each entity
        //           list(int)   - A list of systems that request that component
        //                int    - The System::typeID() of that system
        for (unsigned long i = 0; i < this->entities.size(); i++) {
            Entity e = this->entities[i];
            std::vector<int> components = e.getComponents();

            for (unsigned long j = 0; j < components.size(); j++) {
                int componentIndex = components[j];
                Component *component = this->components[componentIndex];
                std::vector<int> systems = systemsRequestingComponents[component->getTypeID()];

                for (int system : systems) {
                    if (this->systemCache.size() <= system) {
                        this->systemCache.resize(system + 1);
                        if (this->systemCache[system].size() <= i) {
                            this->systemCache[system].resize(i + 1);
                            this->systemCache[system][i].push_back(componentIndex);
                        }
                    }
                }
            }
        }

        // TODO: Create third pass to validate systemCache component query requirements

        this->ecsUpdates = 0;
    }

    for (unsigned long i = 0; i < this->systems.size(); i++) {
        std::vector< std::vector<int> > cache = this->systemCache[i];
        for (unsigned long j = 0; j < cache.size(); j++) {
            std::vector<Component*> components;
            for (int cIndex : cache[j]) {
                components.push_back(this->components[cIndex]);
            }
            this->systems[i]->system(components);
        }
    }

    // std::vector<int> indices;
    // debug("Finding components with type ID %d in %lu entities", typeID, this->entities.size());
    // for (unsigned long i = 0; i < this->entities.size(); i++) {
    //     Entity e = this->entities[i];
    //     debug("Found entity with %lu components", e.getComponents()->size());
    //     for (Component *c : *e.getComponents()) {
    //         debug("Found component with type ID: %d", c->getTypeID());
    //         if (c->getTypeID() == typeID) {
    //             indices.push_back(i);
    //         }
    //     }
    // }

    // return indices;
}

Page::~Page() {

}