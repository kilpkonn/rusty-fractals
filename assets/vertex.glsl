#version 330

#extension GL_ARB_gpu_shader_fp64: enable

attribute vec4 vPosition;

void main() {
    gl_Position = vPosition;
}
