use crate::pong_controller::paddle::{Paddle, PaddleBuilder, PaddleMovementSpeed};
use crate::pong_controller::ball::BallBounds;
use psp::embedded_graphics::Framebuffer;

const DETECTION_X_BASE:i32 = psp::SCREEN_WIDTH as i32 - psp::SCREEN_WIDTH as i32/3; // By default it starts moving towards the ball when it is less than 1/3rd of the screen away.

pub struct Opponent{
    paddle: Paddle,
    difficulty: OpponentDifficulty,
    detection_x: i32,
}

#[derive(Copy, Clone)]
pub enum OpponentDifficulty{
    VeryEasy = 0,
    Easy = 1,
    Normal = 2,
    Hard = 3,
    VeryHard = 4
}

impl Opponent{

    pub fn new(difficulty: OpponentDifficulty) -> Self{

        let speed = match difficulty {
            OpponentDifficulty::VeryEasy => PaddleMovementSpeed::Slow,
            OpponentDifficulty::Easy => PaddleMovementSpeed::Normal,
            OpponentDifficulty::Normal => PaddleMovementSpeed::Normal,
            OpponentDifficulty::Hard => PaddleMovementSpeed::Fast,
            OpponentDifficulty::VeryHard => PaddleMovementSpeed::VeryFast
        };

        let paddle = PaddleBuilder::new()
            .set_default_dimensions()
            .set_speed(speed)
            .set_x(psp::SCREEN_WIDTH as i32 - 15)
            .set_y(psp::SCREEN_HEIGHT as i32/2)
            .build();

        let detection_x = DETECTION_X_BASE - 25*(difficulty as i32); // sets up the detection location (based on difficulty)
        Self {paddle, difficulty, detection_x}
    }

    /// Simple AI that moves its paddle in an attempt to save the ball.
    pub fn step_opponent(&mut self, ball: &BallBounds, disp:&mut Framebuffer){
        if ball.right >= self.detection_x{ // checks to see if the ball is able to be detected
            if ball.top < self.paddle.get_bounds().top{ // If the top of the ball is above the paddle
                self.paddle.move_up(disp);
            } else if ball.bottom > self.paddle.get_bounds().bottom{ // If the ball is below the paddle
                self.paddle.move_down(disp);
            }
        }
    }

    /// Checks to see if a given point is within the paddle.
    pub fn contains(&mut self, x:i32, y:i32) -> bool{
        self.paddle.contains(x,y, false)
    }
}