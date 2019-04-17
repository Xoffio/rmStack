/*
 * @Info:
 *      Answer of StackOverflow motion blur. Fragment shader.
 * 
 * @Author: Ricx8 
 * @Date: 04-17-2019 7:51:00 am 
 * @Last Modified by: Ricx8
 * @Last Modified time: 04-17-2019 8:16:00 am 
 */

// Variables from main
uniform sampler2D u_texture;
uniform float nSamplesF;
uniform float radius;
uniform vec2 dir;

void main(){
    vec2 tc = gl_TexCoord[0].xy;
    float blur = radius;
    float hstep = dir.x;
    float vstep = dir.y;
    float total = 0.0;
    int nSamplesI = int(nSamplesF);

    vec4 sum = vec4(0.0);

    for (int i=1; i<=nSamplesI; i++){
        float floatI = float(i);
        float counter = nSamplesF-floatI+1.0;

        float p = floatI/nSamplesF;
        float tO = (p * 0.1783783784) + 0.0162162162;
        total += tO;

        sum += texture2D(u_texture, vec2(tc.x - counter*blur*hstep, tc.y - counter*blur*vstep)) * tO;
    }
    
    sum += texture2D(u_texture, vec2(tc.x, tc.y)) * 0.2270270270;

    for (int i=nSamplesI; i>=1; i--){
        float floatI = float(i);
        float counter = nSamplesF-floatI+1.0;

        float p = floatI/nSamplesF;
        float tO = (p * 0.1783783784) + 0.0162162162;
        total += tO;

        sum += texture2D(u_texture, vec2(tc.x + counter*blur*hstep, tc.y + counter*blur*vstep)) * tO;
    }

    gl_FragColor =  gl_Color * (sum/total);
}