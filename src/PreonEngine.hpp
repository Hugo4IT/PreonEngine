#pragma once

#include <iostream>
#include <string>
#include <vector>

#ifndef GEN_COMPONENT
#define GEN_COMPONENT(x) static inline int typeID() { return IDCounter<preon::Component>::next<x>(); } inline int getTypeID() { return typeID(); }
#endif

// #ifndef COMPONENT_ID
// #define COMPONENT_ID(x) preon::IDCounter<preon::Component>::next<x>()
// #endif

#ifndef GEN_SYSTEM
#define GEN_SYSTEM(x) static inline int typeID() { return IDCounter<preon::System>::next<x>(); } inline int getTypeID() { return typeID(); }
#endif

// #ifndef SYSTEM_ID
// #define SYSTEM_ID(x) preon::IDCounter<preon::System>::next<x>()
// #endif

namespace preon {
    // Static counter so we can store entities and systems as vectors
    // and reference them by index instead of a hashmap/dictionary.
    template<class A>
    class IDCounter {
    public:
        static int __next() {
            static int currentID = 0;
            return currentID++;
        }

        template<class T>
        static int next() {
            static int id = __next();
            return id;
        };
    };

    struct Vector2 {
    public:
        float x;
        float y;

        Vector2() : x(0),y(0) {}
        Vector2(float xy) : x(xy),y(xy) {}
        Vector2(float x, float y) : x(x),y(y) {}

        const Vector2 yx() const { return Vector2(this->y, this->x); }
        const Vector2 operator+=(const Vector2 rhs) {
            x += rhs.x;
            y += rhs.y;
            return *this;
        }
        const Vector2 operator-=(const Vector2 rhs) {
            x -= rhs.x;
            y -= rhs.y;
            return *this;
        }
        const Vector2 operator*=(const Vector2 rhs) {
            x *= rhs.x;
            y *= rhs.y;
            return *this;
        }
        const Vector2 operator/=(const Vector2 rhs) {
            x /= rhs.x;
            y /= rhs.y;
            return *this;
        }

        const friend Vector2 operator+(Vector2 lhs, const Vector2 rhs) { return lhs += rhs; }
        const friend Vector2 operator-(Vector2 lhs, const Vector2 rhs) { return lhs -= rhs; }
        const friend Vector2 operator*(Vector2 lhs, const Vector2 rhs) { return lhs *= rhs; }
        const friend Vector2 operator/(Vector2 lhs, const Vector2 rhs) { return lhs /= rhs; }
        inline friend std::ostream& operator<<(std::ostream& out, const Vector2& vec) {
            return out << "Vector2(" << vec.x << ", " << vec.y << ")";
        }
    };
    
    class Component {
    public:
        virtual inline int getTypeID() { return IDCounter<Component>::next<Component>(); }
    };

    struct Position : public Component {
        GEN_COMPONENT(Position)
    
    private:
        Vector2 position;
    
    public:
        Position(float x, float y) : position(x, y) {}
        Position(Vector2 position) : position(position) {}
    };
    
    struct Collider : public Component {
        GEN_COMPONENT(Collider)
    
    private:
        Vector2 position;
        Vector2 size;
    
    public:
        Collider(float x, float y, float w, float h) : position(x, y),size(w,h) {}
        Collider(Vector2 position, Vector2 size) : position(position),size(size) {}
    };

    class System {
    public:
        virtual inline int getTypeID() { return IDCounter<System>::next<System>(); }
    };

    class Entity {
    private:
        std::vector<Component*> components;

    public:
        Entity() {}
        void addComponent(Component* component);
        std::vector<Component*> *getComponents();
    };

    class Page {
    private:
        std::string title;
        std::vector<Entity> entities;

    public:
        Page(std::string title);
        ~Page();

        void addEntity(Entity entity);
        std::vector<int> find(int typeID);
        Entity *lastEntity();
    };
}