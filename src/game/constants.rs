use bevy::prelude::*;

// Board dimensions
pub const GAME_BOARD_SIZE_X: usize = 32;
pub const GAME_BOARD_SIZE_Z: usize = 32;

// Tile and node spawn heights
pub const TILE_SPAWN_Y: f32 = 0.0;
pub const NODE_SPAWN_Y: f32 = 0.26;

// render resources
pub const TILE_PATH: &str = "models/Tile.glb#Mesh0/Primitive0";
pub const TILE_COLOR: Color = Color::srgb(0.05, 0.05, 0.08); // matte black
pub const TILE_VFX: Color   = Color::srgb(0.20, 0.20, 0.23);

pub const INTERNET_PATH: &str = "models/Internet.glb#Mesh0/Primitive0";
pub const INTERNET_COLOR: Color = Color::srgb(0.25, 0.22, 0.30);
pub const INTERNET_VFX: Color   = Color::srgb(0.40, 0.37, 0.45);

pub const LOADBALANCER_PATH: &str = "models/LoadBalancer.glb#Mesh0/Primitive0";
pub const LOADBALANCER_COLOR: Color = Color::srgb(0.20, 0.27, 0.26);
pub const LOADBALANCER_VFX: Color   = Color::srgb(0.35, 0.42, 0.41);

pub const FIREWALL_PATH: &str = "models/Firewall.glb#Mesh0/Primitive0";
pub const FIREWALL_COLOR: Color = Color::srgb(0.28, 0.20, 0.20);
pub const FIREWALL_VFX: Color   = Color::srgb(0.43, 0.35, 0.35);

pub const DATABASE_PATH: &str = "models/Database.glb#Mesh0/Primitive0";
pub const DATABASE_COLOR: Color = Color::srgb(0.20, 0.23, 0.30);
pub const DATABASE_VFX: Color   = Color::srgb(0.35, 0.38, 0.45);

pub const COMPUTE_PATH: &str = "models/Compute.glb#Mesh0/Primitive0";
pub const COMPUTE_COLOR: Color = Color::srgb(0.30, 0.28, 0.20);
pub const COMPUTE_VFX: Color   = Color::srgb(0.45, 0.43, 0.35);

pub const STORAGE_PATH: &str = "models/Storage.glb#Mesh0/Primitive0";
pub const STORAGE_COLOR: Color = Color::srgb(0.22, 0.22, 0.24);
pub const STORAGE_VFX: Color   = Color::srgb(0.37, 0.37, 0.39);

pub const QUEUE_PATH: &str = "models/Queue.glb#Mesh0/Primitive0";
pub const QUEUE_COLOR: Color = Color::srgb(0.20, 0.26, 0.30);
pub const QUEUE_VFX: Color   = Color::srgb(0.35, 0.41, 0.45);

pub const CACHE_PATH: &str = "models/Cache.glb#Mesh0/Primitive0";
pub const CACHE_COLOR: Color = Color::srgb(0.20, 0.28, 0.22);
pub const CACHE_VFX: Color   = Color::srgb(0.35, 0.43, 0.37);

pub const CDN_PATH: &str = "models/CDN.glb#Mesh0/Primitive0";
pub const CDN_COLOR: Color = Color::srgb(0.22, 0.22, 0.32);
pub const CDN_VFX: Color   = Color::srgb(0.37, 0.37, 0.47);
