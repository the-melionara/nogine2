#version 330 core

#define MAX_TEXTURES 16

layout(location = 0) out vec4 fCol;

in vec4 vTint;
in vec2 vUV;
flat in uint vTexID;
flat in int vUserData;

uniform sampler2D uTextures[MAX_TEXTURES];

#define TEX_CASE(x) case uint(x): fCol = texture(uTextures[x], vUV) * vTint; break
void main() {
    switch (vTexID) {
        TEX_CASE(0);
        TEX_CASE(1);
        TEX_CASE(2);
        TEX_CASE(3);
        TEX_CASE(4);
        TEX_CASE(5);
        TEX_CASE(6);
        TEX_CASE(7);
        TEX_CASE(8);
        TEX_CASE(9);
        TEX_CASE(10);
        TEX_CASE(11);
        TEX_CASE(12);
        TEX_CASE(13);
        TEX_CASE(14);
        TEX_CASE(15);
        default: fCol = vec4(1.0, 0.0, 1.0, 1.0); break;
    }
}
