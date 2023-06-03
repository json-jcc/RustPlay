#version 460

layout(location = 0) in vec2 raw_position;

layout(location = 0) out vec4 f_color;

void main() {
    f_color = vec4(raw_position.x + 1, raw_position.y + 1, 0.0, 1.0);
}
