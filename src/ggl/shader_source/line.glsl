##vert

attribute vec2 pos;
attribute vec2 end;
attribute vec4 color;

varying vec4 outputColor;

void main(void)
{
	gl_Position = vec4(pos, 0.0, 1.0);
	outputColor = color;
}

##frag

precision highp float;
varying vec4 outputColor;
void main(void)
{
	gl_FragColor = outputColor;
}