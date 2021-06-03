#version 440 core

layout (location = 0) out vec4 vOutColor;

void main() {
    vOutColor = vec4(gl_FragCoord.xy, 1.0, 1.0);
}