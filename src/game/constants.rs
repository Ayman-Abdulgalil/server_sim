use bevy::prelude::*;

// Board dimensions
pub const GAME_BOARD_SIZE_X: usize = 32;
pub const GAME_BOARD_SIZE_Z: usize = 32;

// Tile and node spawn heights
pub const TILE_SPAWN_Y: f32 = 0.0;
pub const NODE_SPAWN_Y: f32 = 0.26;

// render resources
pub const TILE_PATH: &'static str = "models/Tile.glb#Mesh0/Primitive0";
pub const TILE_COLOR: Color = Color::srgb(0.25, 0.25, 0.28);
pub const TILE_EMISSIVE: LinearRgba = LinearRgba::BLACK;

pub const INTERNET_PATH: &'static str = "models/Internet.glb#Mesh0/Primitive0";
pub const INTERNET_COLOR: Color = Color::srgb(0.25, 0.22, 0.30); // muted violet slate
pub const INTERNET_EMISSIVE: LinearRgba = LinearRgba::new(0.02, 0.01, 0.03, 0.0);

pub const LOADBALANCER_PATH: &'static str = "models/LoadBalancer.glb#Mesh0/Primitive0";
pub const LOADBALANCER_COLOR: Color = Color::srgb(0.20, 0.27, 0.26); // dark blue-green slate
pub const LOADBALANCER_EMISSIVE: LinearRgba = LinearRgba::new(0.01, 0.03, 0.02, 0.0);

pub const FIREWALL_PATH: &'static str = "models/Firewall.glb#Mesh0/Primitive0";
pub const FIREWALL_COLOR: Color = Color::srgb(0.28, 0.20, 0.20); // dark warm slate
pub const FIREWALL_EMISSIVE: LinearRgba = LinearRgba::new(0.03, 0.01, 0.01, 0.0);

pub const DATABASE_PATH: &'static str = "models/Database.glb#Mesh0/Primitive0";
pub const DATABASE_COLOR: Color = Color::srgb(0.20, 0.23, 0.30); // cool slate
pub const DATABASE_EMISSIVE: LinearRgba = LinearRgba::new(0.01, 0.02, 0.03, 0.0);

pub const COMPUTE_PATH: &'static str = "models/Compute.glb#Mesh0/Primitive0";
pub const COMPUTE_COLOR: Color = Color::srgb(0.30, 0.28, 0.20); // dark khaki slate
pub const COMPUTE_EMISSIVE: LinearRgba = LinearRgba::new(0.03, 0.03, 0.01, 0.0);

pub const STORAGE_PATH: &'static str = "models/Storage.glb#Mesh0/Primitive0";
pub const STORAGE_COLOR: Color = Color::srgb(0.22, 0.22, 0.24); // near-tile charcoal
pub const STORAGE_EMISSIVE: LinearRgba = LinearRgba::new(0.01, 0.01, 0.01, 0.0);

pub const QUEUE_PATH: &'static str = "models/Queue.glb#Mesh0/Primitive0";
pub const QUEUE_COLOR: Color = Color::srgb(0.20, 0.26, 0.30); // muted teal-slate
pub const QUEUE_EMISSIVE: LinearRgba = LinearRgba::new(0.01, 0.02, 0.03, 0.0);

pub const CACHE_PATH: &'static str = "models/Cache.glb#Mesh0/Primitive0";
pub const CACHE_COLOR: Color = Color::srgb(0.20, 0.28, 0.22); // muted green-slate
pub const CACHE_EMISSIVE: LinearRgba = LinearRgba::new(0.01, 0.03, 0.02, 0.0);

pub const CDN_PATH: &'static str = "models/CDN.glb#Mesh0/Primitive0";
pub const CDN_COLOR: Color = Color::srgb(0.22, 0.22, 0.32); // deep indigo slate
pub const CDN_EMISSIVE: LinearRgba = LinearRgba::new(0.01, 0.01, 0.03, 0.0);
