use super::utils::*;
use super::Ball;
use super::Drawable;
use super::Paddle;
use super::TickResult;
use std::io::{Error, Write};

pub struct GameState {
    pub player: Paddle,
    pub ai: Paddle,
    pub ball: Ball,
    pub ticks: u32,
    pub win_size: Coord,
}

impl GameState {
    pub fn handle_collisions(&mut self) -> bool {
        return true;
    }

    pub fn tick(&mut self) -> TickResult {
        // Check if game over
        if self.ball.game_over() {
            return TickResult::GameOver;
        }

        // Check if ball is hitting paddle
        self.handle_collisions();

        self.ticks += 1;
        TickResult::Ok
    }
}

impl Drawable for GameState {
    fn draw(&mut self, buffer: &mut impl Write) -> Result<(), Error> {
        // Draw board and stuff
        self.player.draw(buffer)?;
        self.ai.draw(buffer)?;
        self.ball.draw(buffer)?;
        Ok(())
    }
}
