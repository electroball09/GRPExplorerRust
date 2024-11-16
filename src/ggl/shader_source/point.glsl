##vert

attribute vec2 pos;
attribute vec4 color;
attribute float size;

varying vec4 outputColor;

void main(void)
{
    gl_Position = vec4(pos.x, pos.y, 0.0, 1.0);
    gl_PointSize = size;
    outputColor = color;
}

##frag

precision highp float;
varying vec4 outputColor;
void main(void)
{
    gl_FragColor = outputColor;
}