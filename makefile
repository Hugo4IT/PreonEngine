CC = g++
CCFLAGS = -MF bin/$*.d -MP -MMD -fPIC -Wall -Wextra -std=c++14
LINKFLAGS = -shared -fPIC
EXT_LIBS = -L/opt/homebrew/Cellar/glfw/3.3.6/lib -lglfw -framework OpenGL
LIB = bin/libpreonengine.dylib

ifeq ($(OS),Windows_NT)
	EXT_LIBS = -lglfw3 -lGL -lX11 -lpthread -lXrandr -lXi -ldl
else
    UNAME_S := $(shell uname -s)
    ifeq ($(UNAME_S),Linux)
        EXT_LIBS = -lglfw3 -lGL -lX11 -lpthread -lXrandr -lXi -ldl
    endif
    ifeq ($(UNAME_S),Darwin)
        EXT_LIBS = -L/opt/homebrew/Cellar/glfw/3.3.6/lib -lglfw -framework OpenGL
    endif
endif

PREFIX = @

SOURCES = $(wildcard src/*.cpp)
DEPENDS = $(patsubst src/%.cpp,bin/%.d,$(SOURCES))
OBJECTS = $(patsubst src/%.cpp,bin/%.o,$(SOURCES))

.PHONY: default
default: debug test

clean:
	@rm -f bin/*.o bin/*.d bin/*.so bin/*.dll bin/*.dylib bin/demo bin/test/*.d bin/test/*.o

debug: CCFLAGS += -g3 -O0 -DDEBUG
debug: build

release: CCFLAGS += -ggdb -O3
release: clean build

bin/%.o: src/%.cpp makefile
	@echo "[BUILDING] $< -> $@"
	$(PREFIX) $(CC) $(CCFLAGS) -c -o $@ $<

-include $(DEPENDS)

preBuild:
	@echo "[INFO] Preparing build"
	@mkdir -p bin bin/test
initBuild: preBuild $(LIB)
postBuild: initBuild
	@echo "[INFO] Copying headers"
	@cp -f $(wildcard src/*.hpp) inc/PreonEngine/
	@echo "[INFO] Done!"
build: postBuild

$(LIB): $(OBJECTS)
	@echo "[LINKING] $^ -> $@"
	$(PREFIX) $(CC) $(LINKFLAGS) $(EXT_LIBS) -o $@ $^

TEST_SOURCES = $(wildcard test/*.cpp)
TEST_DEPENDS = $(patsubst test/%.cpp,bin/test/%.d,$(TEST_SOURCES))
TEST_OBJECTS = $(patsubst test/%.cpp,bin/test/%.o,$(TEST_SOURCES))
TEST_CCFLAGS = -MF bin/test/$*.d -MP -MMD -I inc
TEST_LINKFLAGS = -Lbin -l preonengine

bin/test/%.o: test/%.cpp makefile
	@echo "[BUILDING] $< -> $@"
	$(PREFIX) $(CC) $(TEST_CCFLAGS) -c -o $@ $<

-include $(TEST_DEPENDS)

test: $(TEST_OBJECTS)
	@echo "[LINKING] $^ -> bin/demo"
	$(PREFIX) $(CC) $(TEST_LINKFLAGS) $(EXT_LIBS) -o bin/demo $^
	@echo "[INFO] Running..."
	$(PREFIX) ./bin/demo