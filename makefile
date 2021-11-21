# BUILD SPIR-V SHADERS

VERT = $(wildcard preon_module_wgpu/src/shaders/*.vert)
VERT_OUT = $(patsubst %,%.spv,$(VERT))
FRAG = $(wildcard preon_module_wgpu/src/shaders/*.frag)
FRAG_OUT = $(patsubst %,%.spv,$(FRAG))

preon_module_wgpu/src/shaders/%.vert.spv: preon_module_wgpu/src/shaders/%.vert makefile
	glslangValidator -V $< -o $<.spv
	naga $< $<.wgsl

preon_module_wgpu/src/shaders/%.frag.spv: preon_module_wgpu/src/shaders/%.frag makefile
	glslangValidator -V $< -o $<.spv
	naga $< $<.wgsl

shaders: $(VERT_OUT) $(FRAG_OUT)
