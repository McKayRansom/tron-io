#version 100
attribute vec3 position;
attribute vec2 texcoord;
attribute vec4 color0;
attribute vec4 normal;

varying lowp vec2 uv;
varying lowp vec4 color;

/*
 * Macroquad seems to know about 'Model' and 'Projection' uniforms
 *
 * and it creates position, texcoord, color0, normal at some point as well
 *
 */
uniform mat4 Model;
uniform mat4 Projection;

void main() {
  gl_Position = Projection * Model * vec4(position, 1);
  color = color0 / 255.0;
  uv = texcoord;
}