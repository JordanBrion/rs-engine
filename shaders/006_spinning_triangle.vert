#version 440 core

layout (location = 1) in vec3 vPosition;

layout (binding = 5) uniform Matrices {
    mat4 mModel;
} matrices;

void main() {
    gl_Position = matrices.mModel * vec4(vPosition, 1.0);
}