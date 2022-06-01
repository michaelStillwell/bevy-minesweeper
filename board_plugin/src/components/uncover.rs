use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component, Inspectable)]
pub struct Uncover;
