#version 460


layout(location = 0) in vec2 position;

layout(location = 0) out vec2 raw_position;


void main() {
    gl_Position = vec4(position, 0.0, 1.0);
}