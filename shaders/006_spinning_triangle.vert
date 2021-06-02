#version 440 core

layout (location = 1) in vec3 vPosition;

layout (binding = 5) uniform Matrices {
    mat4 mModel;
    mat4 mView;
    mat4 mProjection;
} matrices;

void main() {
    gl_Position = matrices.mProjection * matrices.mView * matrices.mModel * vec4(vPosition, 1.0);
}