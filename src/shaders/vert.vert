// OpenGL 4.6
#version 460 core

// Input vertex data, different for all executions of this shader.
// attribute is replaced with in 
layout (location = 0) in vec2 position;
layout (location = 1) in vec4 color;

out vec4 outColor;

void main(){
    gl_Position = vec4(vec3(position, 0.0), 1.0); // Set position
    // why vec4 for gl_Position?
    // https://stackoverflow.com/questions/12666557/why-do-we-need-to-set-gl-position
    outColor = color;
}