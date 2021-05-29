#version 440 core

layout (location = 1) in vec3 vPosition;
layout (location = 2) in vec3 vInColor;

layout (location = 0) out vec3 vOutColor;

layout (binding = 5) uniform Matrices {
    mat4 mModel;
    mat4 mView;
    mat4 mProjection;
} matrices;

void main() {
    vOutColor = vInColor;
    gl_Position = matrices.mProjection * matrices.mView * matrices.mModel * vec4(vPosition, 1.0);
}