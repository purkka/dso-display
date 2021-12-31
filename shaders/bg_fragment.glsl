#version 140
#define PI 3.141592

out vec4 color;

uniform float time;
uniform uint height;

void main() {
    vec2 uv = gl_FragCoord.xy / height;
    uv.y += time / 8;

    // variables
    float split = 8;
    float ncol = 2;

    // the slope
    uv.y -= tan(PI / 4) * uv.x;

    // colors every other stripe
    if (mod(floor(uv.y * split), ncol) == 0) {
        color = vec4(0.25, 0.25, 0.25, 1);
    } else { // keep transparent
        color = vec4(0);
    }
}
