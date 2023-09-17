#version 460 core

in vec4 outColor;
out vec4 fragColor;

void main(){
    // fragColor = outColor;
    fragColor = vec4(0.0f, 1.0f, 0.0f, 1.0f);
}