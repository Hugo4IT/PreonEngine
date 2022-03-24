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
    this->components.push_back(component);
    this->ecsUpdates++;

    return this->components.size() - 1;
}

void Page::addSystem(System *system) {
    this->systems.push_back(system);
    this->ecsUpdates++;
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
            
            std::cout << s->getTypeID() << std::endl;

            std::vector<int> requestedComponents = s->query();

            unsigned long highestIndex = 0;
            for (int r : requestedComponents)
                if ((unsigned long)r > highestIndex)
                    highestIndex = r;

            if (systemsRequestingComponents.size() <= highestIndex)
                systemsRequestingComponents.resize(highestIndex + 1, std::vector<int>());
            
            for (int r : requestedComponents) {
                systemsRequestingComponents[r].push_back(i);
            }
        }

        // Second pass: Create systemCache:
        // list(list(list(int))) - Indexed by System::typeID();
        //      list(list(int))  - List of entities that apply to that system
        //           list(int)   - A list of the components that will be used
        //                int    - Indices of the components in this->components
        for (unsigned long i = 0; i < this->entities.size(); i++) {
            Entity e = this->entities[i];
            std::vector<int> components = e.getComponents();

            for (unsigned long j = 0; j < components.size(); j++) {
                int componentIndex = components[j];
                Component *component = this->components[componentIndex];
                if (component->getTypeID() < systemsRequestingComponents.size()) {
                    std::vector<int> systems = systemsRequestingComponents[component->getTypeID()];

                    for (int system : systems) {
                        if (this->systemCache.size() <= system)
                            this->systemCache.resize(system + 1);
                        if (this->systemCache[system].size() <= i)
                            this->systemCache[system].resize(i + 1);
                        this->systemCache[system][i].push_back(componentIndex);
                    }
                }
            }
        }

        // TODO: Create third pass to validate systemCache component query requirements

        this->ecsUpdates = 0;
    }

    for (unsigned long i = 0; i < this->systems.size(); i++) {
        if (i < this->systemCache.size()) {
            std::vector< std::vector<int> > cache = this->systemCache[i];
            for (unsigned long j = 0; j < cache.size(); j++) {
                std::vector<Component*> components;
                for (int cIndex : cache[j]) {
                    components.push_back(this->components[cIndex]);
                }
                if (components.size() > 0) {
                    this->systems[i]->system(components);
                }
            }
        }
    }
}

Entity *Page::lastEntity() {
    return &this->entities[this->entities.size() - 1];
}

Page::~Page() {

}