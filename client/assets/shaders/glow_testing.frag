#version 100
precision lowp float;

varying vec4 color;
varying vec2 uv;

uniform sampler2D Texture;

// https://www.shadertoy.com/view/XtlSD7

vec2 CRTCurveUV(vec2 uv)
{
    uv = uv * 2.0 - 1.0;
    vec2 offset = abs( uv.yx ) / vec2( 6.0, 4.0 );
    uv = uv + uv * offset * offset;
    uv = uv * 0.5 + 0.5;
    return uv;
}

void DrawVignette( inout vec3 color, vec2 uv )
{
    float vignette = uv.x * uv.y * ( 1.0 - uv.x ) * ( 1.0 - uv.y );
    vignette = clamp( pow( 16.0 * vignette, 0.3 ), 0.0, 1.0 );
    color *= vignette;
}


void DrawScanline( inout vec3 color, vec2 uv )
{
    float iTime = 0.1;
    float scanline 	= clamp( 0.95 + 0.05 * cos( 3.14 * ( uv.y + 0.008 * iTime ) * 240.0 * 1.0 ), 0.0, 1.0 );
    float grille 	= 0.85 + 0.15 * clamp( 1.5 * cos( 3.14 * uv.x * 640.0 * 1.0 ), 0.0, 1.0 );
    color *= scanline * grille * 1.2;
}

// TODO: I guessed at const is that a real thing
const float glow_threshold = .5;
const float glow_distance = 0.0010;

void main() {
    // vec2 crtUV = CRTCurveUV(uv);
    // vec3 res = texture2D(Texture, uv).rgb * color.rgb;
    // if (crtUV.x < 0.0 || crtUV.x > 1.0 || crtUV.y < 0.0 || crtUV.y > 1.0)
    // {
    //     res = vec3(0.0, 0.0, 0.0);
    // }
    // DrawVignette(res, crtUV);
    // DrawScanline(res, uv);
    gl_FragColor = color * texture2D(Texture, uv);
    if (gl_FragColor.r <= glow_threshold && gl_FragColor.g <= glow_threshold && gl_FragColor.b <= glow_threshold) {
        vec4 sum = vec4(0.0, 0.0, 0.0, 0.0);
        for (int n = 0; n < 9; ++n) {
            // uv_y = (tex_coord.y * size.y) + (glow_size * float(n - 4.5));
            // float h_sum = 0.0;
            vec4 h_sum = vec4(0.0, 0.0, 0.0, 0.0);
            h_sum += color * texture2D(Texture, uv + vec2(glow_distance, 0.0) * vec2(n, n));
            h_sum += color * texture2D(Texture, uv + vec2(-glow_distance, 0.0) * vec2(n, n));
            h_sum += color * texture2D(Texture, uv + vec2(0.0, glow_distance) * vec2(n, n));
            h_sum += color * texture2D(Texture, uv + vec2(0.0, -glow_distance) * vec2(n, n));
            // sum += vec4(1.0, 0.0, 0.0, 0.0);

            // h_sum += texelFetch(t0, ivec2(uv_x - (4.0 * glow_size), uv_y), 0).a;
            // h_sum += texelFetch(t0, ivec2(uv_x - (3.0 * glow_size), uv_y), 0).a;
            // h_sum += texelFetch(t0, ivec2(uv_x - (2.0 * glow_size), uv_y), 0).a;
            // h_sum += texelFetch(t0, ivec2(uv_x - glow_size, uv_y), 0).a;
            // h_sum += texelFetch(t0, ivec2(uv_x, uv_y), 0).a;
            // h_sum += texelFetch(t0, ivec2(uv_x + glow_size, uv_y), 0).a;
            // h_sum += texelFetch(t0, ivec2(uv_x + (2.0 * glow_size), uv_y), 0).a;
            // h_sum += texelFetch(t0, ivec2(uv_x + (3.0 * glow_size), uv_y), 0).a;
            // h_sum += texelFetch(t0, ivec2(uv_x + (4.0 * glow_size), uv_y), 0).a;
            sum += h_sum / 4.0;
        }
        gl_FragColor = sum / 9.0;
    }

}