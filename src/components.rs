use bevy::prelude::*;

#[derive(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

impl Health {

    pub fn new(max: f32) -> Self {
        Self {
            current: max,
            max,
        }
    }

    pub fn is_alive(&self) -> bool {
        self.current > 0.0
    }

    pub fn heal(&mut self, points: f32) {
        self.current = (self.current + points).min(self.max);
    }

    pub fn take_damage(&mut self, points: f32) {
        self.current = (self.current - points).max(0.0);
    }

    pub fn set_max(&mut self, max: f32) {
        self.max = max;
        self.current = self.current.min(max);
    }
}

#[derive(Component)]
pub struct Position(pub Vec3);

#[derive(Component)]
pub struct Velocity(pub Vec3);

