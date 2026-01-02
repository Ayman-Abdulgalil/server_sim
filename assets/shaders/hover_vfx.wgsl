// assets/shaders/hover_outline_glow.wgsl

#import bevy_pbr::{
    mesh_view_bindings::globals,
    forward_io::VertexOutput,
}

// Minimal per-material data: only hover fade (0..1).
// Everything else is constants in the shader (including your color).
struct HoverMaterial {
    hover: f32,
    // pad to 16-byte alignment for uniforms
    _pad0: vec3<f32>,
};

@group(2) @binding(0)
var<uniform> material: HoverMaterial;

// Your requested color: LinearRgba::new(0.25, 0.8, 1.0, 1.0)
const HOVER_COLOR: vec4<f32> = vec4<f32>(0.25, 0.8, 1.0, 1.0);

// Tweakables kept internal (not exposed as parameters).
const OUTLINE_STRENGTH: f32 = 0.55; // thin outline intensity
const GLOW_STRENGTH: f32 = 0.35;    // base glow intensity
const PULSE_SPEED: f32 = 2.2;       // radians-ish scale for sin()
const PULSE_AMOUNT: f32 = 0.25;     // how much the glow pulses

// Fresnel helper: higher at grazing angles (edge of object).
fn fresnel(n: vec3<f32>, v: vec3<f32>, power: f32) -> f32 {
    let ndv = clamp(dot(normalize(n), normalize(v)), 0.0, 1.0);
    return pow(1.0 - ndv, power);
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    // View direction: from fragment toward camera in world space.
    let V = normalize(globals.camera_position - in.world_position.xyz);
    let N = normalize(in.world_normal);

    // Edge factor for a thin outline-ish highlight (fresnel).
    let edge = fresnel(N, V, 4.0);

    // Gentle pulse using Bevy globals time since startup.
    let pulse = 0.5 + 0.5 * sin(globals.time * PULSE_SPEED);

    // Hover fade controlled from Rust (0..1).
    let h = clamp(material.hover, 0.0, 1.0);

    // Outline + glow are both gated by hover fade.
    let outline = edge * OUTLINE_STRENGTH * h;
    let glow = edge * GLOW_STRENGTH * (1.0 + pulse * PULSE_AMOUNT) * h;

    // Additive emissive-style tint on top of the underlying lit color.
    // Minimal approach: output just the effect color with alpha = max(outline, glow).
    // (In a real material extension, you'd add this to the base PBR color instead.)
    let a = clamp(max(outline, glow), 0.0, 1.0);
    let rgb = HOVER_COLOR.rgb * (outline + glow);

    return vec4<f32>(rgb, a);
}
