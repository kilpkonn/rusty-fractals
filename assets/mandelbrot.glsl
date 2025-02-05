#version 330

#extension GL_ARB_gpu_shader_fp64: enable

precision highp float;
uniform mat4 uMVPMatrix;
uniform vec2 uWindowSize;
uniform float uTimeDiff;

void main() {
    //Scale point by input transformation matrix
    vec2 p = (uMVPMatrix * vec4(gl_FragCoord.xy / uWindowSize, 0, 1)).xy;
    vec2 c = p;

    int total = -999;

    //Set default color to HSV value for black
    vec3 color=vec3(0.0, 0.0, 0.0);
    for (int x=-1;x<=1;x++) {
        for (int y=-1;y<=1;y++) {
            vec2 p = (uMVPMatrix * vec4((gl_FragCoord.xy + vec2(x, y)) / uWindowSize, 0, 1)).xy;
            vec2 c = p;
            for (int i=0;i<900;i++){
                //Perform complex number arithmetic
                p= vec2(p.x*p.x-p.y*p.y, 2.0*p.x*p.y)+c;

                if (dot(p, p)>4.0){
                    if (total < 0){
                        total = 0;
                    }
                    total += i; // / (1 + x*x*y*y);
                    break;
                }
            }
        }
    }

    //The point, c, is not part of the set, so smoothly color it. colorRegulator increases linearly by 1 for every extra step it takes to break free.
    float colorRegulator = float(total/9.0)-log(log(total+2))/100.0;
    //This is a coloring algorithm I found to be appealing. Written in HSV, many functions will work.
    color = vec3(0.5 + sin(uTimeDiff/5000) / 2 + .012*colorRegulator, 1.0, .2+.4*(1.0+sin(.3*colorRegulator)));

    //Change color from HSV to RGB. Algorithm from https://gist.github.com/patriciogonzalezvivo/114c1653de9e3da6e1e3
    vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
    vec3 m = abs(fract(color.xxx + K.xyz) * 6.0 - K.www);
    gl_FragColor.rgb = color.z * mix(K.xxx, clamp(m - K.xxx, 0.0, 1.0), color.y);

    gl_FragColor.a=1.0;
}
