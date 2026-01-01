use bevy::prelude::*;

pub const BOARD_SIZE_X: usize = 32;
pub const BOARD_SIZE_Y: usize = 32;

pub const TILE_Y: f32 = 0.0;
pub const NODE_Y: f32 = 0.26;

/// How much the tile rises when hovered.
pub const TILE_HOVER_LIFT_Y: f32 = 0.06;

// Raycast plane slightly above the ground to match tile/node height logic.
pub const CLICK_PLANE_Y: f32 = 0.25;

// FX rings
pub const TILE_HOVER_RING_RADIUS: f32 = 0.48;
pub const TILE_HOVER_RING_Y: f32 = 0.03;

pub const NODE_SELECT_RING_RADIUS: f32 = 0.40;
pub const NODE_SELECT_RING_Y: f32 = 0.02;

// Smoothness (bigger = snappier). This “exponential lerp” style is frame-rate independent.
pub const TILE_HOVER_SPEED: f32 = 14.0;

// Base and hovered colors (tweak as desired)
pub const TILE_BASE_COLOR: Color = Color::srgb(0.25, 0.25, 0.28);
pub const TILE_HOVER_COLOR: Color = Color::srgb(0.35, 0.35, 0.42);

// Emissive (glow) targets
pub const TILE_BASE_EMISSIVE: LinearRgba = LinearRgba::BLACK;
pub const TILE_HOVER_EMISSIVE: LinearRgba = LinearRgba::rgb(0.9, 0.9, 0.2);
