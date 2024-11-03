#version 330 core

#define MAX_TEXTURES 16

layout(location = 0) out vec4 fCol;

in vec4 vTint;
in vec2 vUV;

uniform sampler2D uTextures[MAX_TEXTURES];

void main() {
    fCol = texture(uTextures[0], vUV) * vTint;
}
