#version 330 core

layout(location = 0) in vec2 aPos;
layout(location = 1) in vec4 aTint;
layout(location = 2) in vec2 aUV;

out vec4 vTint;
out vec2 vUV;

void main() {
    gl_Position = vec4(aPos, 0.0, 1.0);

    vTint = aTint;
    vUV = aUV;
}
