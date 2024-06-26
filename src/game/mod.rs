use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;
use sdl2::render::WindowCanvas;
use crate::characters::CharacterType;
use crate::assets::geometry::SpriteAsset;
use crate::config::PhysicsConfig;
use crate::game::action::Direction;
use crate::game::default::DefaultGame;
use crate::game::event::GameEvent;
use crate::game::physics::Body;
use crate::game::scale::PhysicsScale;
use crate::game::sync::AsyncGame;

pub mod event;
pub mod physics;
mod physics_debug;
pub mod polygon;
pub mod scale;
pub mod action;
mod sync;
mod default;

pub trait Game {
    fn push(&mut self, direction: Direction);
    fn spawn_asset(&mut self, sprite: SpriteAsset);
    fn spawn_character(&mut self, character: CharacterType);
    fn destroy(&mut self, id: u128);
    fn explosion(&mut self);
    fn update(&mut self, delta: Duration) -> Vec<GameEvent>;
    fn bodies(&self) -> Vec<Body>;
    fn debug_draw(&self);
}

pub fn game<C: Into<Option<Rc<RefCell<WindowCanvas>>>>>(
    scale: PhysicsScale,
    physics_config: PhysicsConfig,
    canvas: C
) -> Box<dyn Game> {
    if physics_config.debug_draw {
        Box::new(DefaultGame::new(scale, physics_config, canvas))
    } else {
        Box::new(AsyncGame::new(scale, physics_config))
    }
}