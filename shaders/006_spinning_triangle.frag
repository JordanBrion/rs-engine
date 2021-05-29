#version 440 core

layout (location = 0) in vec3 vInColor;

layout (location = 0) out vec4 vOutColor;

void main() {
    vOutColor = vec4(vInColor, 1.0);
}