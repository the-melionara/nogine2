#version 330 core

layout(location = 0) in vec2 aPos;
layout(location = 1) in uint aTint;
layout(location = 2) in vec2 aUV;

// Bit layout:
// - **4 bits:** `tex_id`
// - **2 bits:** `uv1`
// - **10 bits:** unused
// - **16 bits:** `user_data`
layout(location = 3) in uint aCtrl;

out vec4 vTint;
out vec2 vUV;
out vec2 vUV1;
flat out uint vTexID;
flat out uint vUserData;

uniform mat3 uViewMat;

void main() {
    gl_Position = vec4((uViewMat * vec3(aPos, 1.0)).xy, 0.0, 1.0);

    vTint = vec4(
        float(aTint & 0xFFu),
        float((aTint >> 8) & 0xFFu),
        float((aTint >> 16) & 0xFFu),
        float((aTint >> 24) & 0xFFu)
    ) / 255.0;
    vUV = aUV;
    vTexID = aCtrl & 0xFu;
    vUV1 = vec2(
        float((aCtrl >> 2) & 1u),
        float((aCtrl >> 3) & 1u)
    );
    vUserData = aCtrl >> 16;
}
