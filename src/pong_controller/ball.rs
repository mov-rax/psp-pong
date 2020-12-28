use embedded_graphics::style::{Styled, PrimitiveStyle, PrimitiveStyleBuilder};
use embedded_graphics::primitives::Circle;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::{RgbColor, Point, Primitive, Transform, Dimensions};
use psp::embedded_graphics::Framebuffer;
use embedded_graphics::drawable::Drawable;
use rand;
use rand::{RngCore, Error, SeedableRng, Rng};
use rand::prelude::SmallRng;
use rand_chacha::{ChaChaRng, ChaCha20Rng};

static mut MOVEMENT_MAGNITUDE:i32 = 2; // defaults to 2

pub struct BallBounds{
    pub top: i32,
    pub bottom: i32,
    pub left: i32,
    pub right: i32
}

pub struct Ball{
    circle:Styled<Circle, PrimitiveStyle<Rgb888>>,
    background:Styled<Circle, PrimitiveStyle<Rgb888>>,
    bounds: BallBounds,
    direction: Point,
    speed_counter: u32,
}

pub enum BallAxes{
    Vertical,
    Horizontal
}


impl Ball{
    pub fn new(x:i32,y:i32,radius:u32, movement_magnitude:u32, rng: &mut ChaCha20Rng) -> Self{
        let circle_style = PrimitiveStyleBuilder::new()
            .fill_color(Rgb888::WHITE)
            .build();
        let background_style = PrimitiveStyleBuilder::new()
            .fill_color(Rgb888::BLACK)
            .build();
        let center = Point::new(x,y);
        let circle = Circle::new(center, radius).into_styled(circle_style);
        let background = Circle::new(center, radius).into_styled(background_style);
        let bounds = BallBounds { top: y,
            bottom: circle.primitive.bottom_right().y,
            left: x,
            right: circle.primitive.bottom_right().x
        };

        let direction= Self::gen_direction(rng);
        unsafe {
            MOVEMENT_MAGNITUDE = movement_magnitude as i32;
        }


        Self {circle, background, bounds, direction, speed_counter: 0}
    }

    /// Returns a random direction.
    fn gen_direction(rng: &mut ChaCha20Rng) -> Point{
        let seed = rng.gen_range(0..=3);
        unsafe {
            match seed { // random number from 0-3 will appear, leading to the ball going in a random direction
                0 => Point::new(-MOVEMENT_MAGNITUDE, -MOVEMENT_MAGNITUDE),
                1 => Point::new(-MOVEMENT_MAGNITUDE, MOVEMENT_MAGNITUDE),
                2 => Point::new(MOVEMENT_MAGNITUDE, -MOVEMENT_MAGNITUDE),
                3 => Point::new(MOVEMENT_MAGNITUDE, MOVEMENT_MAGNITUDE),
                _ => Point::new(0,0)
            }
        }
    }
    /// Blackens out the circle.
    pub fn blacken(&mut self, disp: &mut Framebuffer) {
        self.background.draw(disp);
    }

    pub fn get_direction(&self) -> (i32, i32){
        (self.direction.x, self.direction.y)
    }

    // Updates the now-current bounds of the circle.
    fn update_bounds(&mut self){
        self.bounds.top = self.circle.primitive.top_left().y;
        self.bounds.left = self.circle.primitive.top_left().x;
        self.bounds.bottom = self.circle.primitive.bottom_right().y;
        self.bounds.right = self.circle.primitive.bottom_right().x;
    }

    /// Moves the ball to a Point
    pub fn move_ball(&mut self, location:Point, disp: &mut Framebuffer){
        self.background.draw(disp);
        self.background.translate_mut(location);
        self.circle.translate_mut(location);
        self.circle.draw(disp);
        self.update_bounds();
    }

    /// Moves the ball in the direction it is going towards
    pub fn step_direction(&mut self, disp: &mut Framebuffer){
        self.move_ball(self.direction, disp);
    }

    /// Flips the direction the ball is going towards
    pub fn flip_direction(&mut self, axes: BallAxes){
        match axes{
            BallAxes::Horizontal => {
                self.speed_counter += 1;
                self.direction.x *= -1;
                if self.speed_counter % 3 == 0{ // Every third horizontal flip it will increase the speed of the ball by 1
                    if self.direction.x > 0{ // makes the ball move faster on the x-axis
                        self.direction.x += 1;
                    } else{
                        self.direction.x -= 1;
                    }
                }
            },
            BallAxes::Vertical => {
                self.direction.y *= -1;
            }
        }
    }

    /// Gets an immutable reference to the bounds of the ball
    pub fn get_bounds(&self) -> &BallBounds{
        &self.bounds
    }

}

