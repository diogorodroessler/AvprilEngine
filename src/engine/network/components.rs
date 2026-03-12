use bevy::prelude::*;
use serde::{
    Deserialize,
    Serialize
};

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct Player;

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}