#version 140

in vec3 position;

uniform float x;

void main() {
    vec3 pos = position;
    pos.x += x;
    gl_Position = vec4(pos, 1.0);
}