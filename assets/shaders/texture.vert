#version 140

in vec3 position;
in vec2 tex_coords;

out vec2 v_tex_coords;

uniform mat4 view_projection;
uniform mat4 model;

void main() {
    v_tex_coords = tex_coords;
    gl_Position = view_projection * model * vec4(position, 1.0);
}