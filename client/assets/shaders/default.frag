#version 100

// Macroquad seems to create 'Texture' and '_ScreenTexture' images (but I guess
// they show as uniforms?)
varying lowp vec4 color;
varying lowp vec2 uv;

uniform sampler2D Texture;

void main() { gl_FragColor = color * texture2D(Texture, uv); }