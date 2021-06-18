#version 330 compatibility
#define MAX_VERTICES 256

layout(points) in;
layout(points, max_vertices=MAX_VERTICES) out;

uniform float shift_r;
uniform float shift_x;
uniform float zoom;

float update_x(float x, float r)
{
    return r * x * (1.0f - x);
}

void bifurcate(float r, float zoom, float x_shift)
{
    // initial guess
    float x = 0.5f;

    // throw away first N values in the series
    for (int i = 0; i < 2 * MAX_VERTICES; ++i)
    {
        x = update_x(x, r);
    }

    // emit point only if it lands inside viewspace
    int num_emited = 0;

    while (num_emited < MAX_VERTICES)
    {
        x = update_x(x, r);

        float pos = (x - x_shift) / zoom;

        if (pos <= -1.0f && pos <= 1.0f)
        {
            num_emited++;
            gl_Position = gl_in[0].gl_Position + vec4(0.0f, pos, 0.0f, 0.0f);
            EmitVertex();
            EndPrimitive();
        }
    }
}

void main()
{
    float r = zoom * gl_in[0].gl_Position.x + shift_r;

    bifurcate(r, zoom, shift_x);
}