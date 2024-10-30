#version 330 core

layout(location = 0) in vec2 aPos;
layout(location = 1) in vec4 aTint;
layout(location = 2) in vec2 aUV;
layout(location = 3) in uint aTexID;
layout(location = 4) in int aUserData;

out vec4 vTint;
out vec2 vUV;
flat out uint vTexID;
flat out int vUserData;

uniform mat3 uViewMat;

void main() {
    gl_Position = vec4((uViewMat * vec3(aPos, 1.0)).xy, 0.0, 1.0);

    vTint = aTint;
    vUV = aUV;
    vTexID = aTexID;
    vUserData = aUserData;
}
