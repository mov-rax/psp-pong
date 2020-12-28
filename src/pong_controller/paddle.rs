use embedded_graphics::prelude::{Point, Primitive, Transform, Dimensions};
use psp::embedded_graphics::Framebuffer;
use embedded_graphics::primitives::Rectangle;
use embedded_graphics::style::{Styled, PrimitiveStyle, PrimitiveStyleBuilder};
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::pixelcolor::RgbColor;
use embedded_graphics::drawable::Drawable;

pub struct Paddle{
    rect: Styled<Rectangle, PrimitiveStyle<Rgb888>>,
    background: Styled<Rectangle, PrimitiveStyle<Rgb888>>,
    bounds: PaddleBounds,
    speed_up: Point, // Speed of the paddle moving up the screen
    speed_down: Point, // Speed of the paddle moving down the screen
}

#[derive(Copy, Clone)]
pub enum PaddleMovementSpeed{
    Slow = 2,
    Normal = 4,
    Fast = 6,
    VeryFast = 8,
}

pub struct PaddleBuilder{
    x: Option<i32>,
    y: Option<i32>,
    thickness: Option<u32>,
    height: Option<u32>,
    speed: Option<PaddleMovementSpeed>
}

impl PaddleBuilder{
    pub fn new() -> Self{
       Self{ x: None, y: None, thickness: None, height: None, speed: None}
    }

    pub fn set_default_dimensions(mut self) -> Self{
        self.thickness = Some(10);
        self.height = Some(40);
        self
    }

    pub fn set_thickness(mut self, thickness:u32) -> Self{
        self.thickness = Some(thickness);
        self
    }

    pub fn set_height(mut self, height:u32) -> Self{
        self.height = Some(height);
        self
    }

    pub fn set_x(mut self, x:i32) -> Self{
        self.x = Some(x);
        self
    }

    pub fn set_y(mut self, y:i32) -> Self{
        self.y = Some(y);
        self
    }

    pub fn set_speed(mut self, speed: PaddleMovementSpeed) -> Self{
        self.speed = Some(speed);
        self
    }

    pub fn build(self) -> Paddle{
        if let Some(thickness) = self.thickness{
            if let Some(height) = self.height{
                if let Some(x) = self.x{
                    if let Some(y) = self.y{
                        if let Some(speed) = self.speed{
                            return Paddle::new(x,y,thickness,height,speed)
                        }
                    }
                }
            }
        }
        panic!("The PaddleBuilder was not given enough information to create a Paddle.")
    }
}

pub struct PaddleBounds{
    pub top: i32,
    pub bottom: i32,
    pub left: i32,
    pub right: i32
}

impl Paddle {
    pub fn new(x:i32,y:i32,thickness:u32,height:u32, speed:PaddleMovementSpeed) -> Self{

        let style = PrimitiveStyleBuilder::new()
            .fill_color(Rgb888::WHITE)
            .build();
        let background_style = PrimitiveStyleBuilder::new()
            .fill_color(Rgb888::BLACK)
            .build();
        let point1 = Point::new(x,y);
        let point2 = Point::new(x+thickness as i32,y+height as i32);
        let rect = Rectangle::new(point1,point2)
            .into_styled(style);
        let background = Rectangle::new(point1, point2)
            .into_styled(background_style);

        let bounds = PaddleBounds {
            top: rect.primitive.top_left().y,
            bottom: rect.primitive.bottom_right().y,
            left: rect.primitive.top_left().x,
            right: rect.primitive.bottom_right().x};

        let speed_up = Point::new(0,-(speed as i32));
        let speed_down = Point::new(0,speed as i32);
        Self {rect, background, bounds, speed_up, speed_down}
    }

    /// Updates the now-current bounds of the paddle.
    fn update_bounds(&mut self){
        self.bounds.top = self.rect.primitive.top_left().y;
        self.bounds.bottom = self.rect.primitive.bottom_right().y;
        self.bounds.left = self.rect.primitive.top_left().x;
        self.bounds.right = self.rect.primitive.bottom_right().x;
    }

    /// Redraws the paddle
    pub fn redraw(&mut self, disp: &mut Framebuffer){
        self.rect.draw(disp);
    }

    /// Moves the paddle up
    pub fn move_up(&mut self, disp: &mut Framebuffer){
        self.background.draw(disp);
        self.rect.translate_mut(self.speed_up);
        self.background.translate_mut(self.speed_up);
        self.rect.draw(disp);
        self.update_bounds();
    }

    /// Moves the paddle down
    pub fn move_down(&mut self, disp: &mut Framebuffer){
        self.background.draw(disp);
        self.rect.translate_mut(self.speed_down);
        self.background.translate_mut(self.speed_down);
        self.rect.draw(disp);
        self.update_bounds();
    }
    /// Checks to see if a given point is within the paddle.
    pub fn contains(&self, x:i32, y:i32, is_player:bool) -> bool {
        if is_player{
            if  x < self.rect.primitive.bottom_right.x &&
                y > self.rect.primitive.top_left.y &&
                y < self.rect.primitive.bottom_right.y{
                return true
            }
        } else {
            if  x > self.rect.primitive.top_left.x &&
                y > self.rect.primitive.top_left.y &&
                y < self.rect.primitive.bottom_right.y{
                return true
            }
        }

        false
    }

    pub fn get_bounds(&self) -> &PaddleBounds { &self.bounds }
}