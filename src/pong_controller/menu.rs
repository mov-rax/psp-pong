
use psp::embedded_graphics::Framebuffer;
use embedded_graphics::style::{PrimitiveStyleBuilder, TextStyleBuilder, Styled, TextStyle, PrimitiveStyle};
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::{RgbColor, Point, Primitive, Dimensions, Transform, Drawable};
use embedded_graphics::primitives::Rectangle;
use psp::{SCREEN_WIDTH, SCREEN_HEIGHT};
use embedded_graphics::fonts::{Font12x16, Text, Font};
use crate::pong_controller::opponent::OpponentDifficulty;

#[feature(min_const_generics)]
pub struct Menu<'a, T, U, const N:usize>
where T: Font + Copy + Clone,
    U: Font + Copy + Clone,
{
    menu_items: [Styled<Text<'a>, TextStyle<Rgb888, T>>; N],
    menu_title: Styled<Text<'a>, TextStyle<Rgb888, U>>,
    selected_counter:usize,
    menu_background: Styled<Rectangle, PrimitiveStyle<Rgb888>>,
    menu_foreground: Styled<Rectangle, PrimitiveStyle<Rgb888>>,
    selected_style: TextStyle<Rgb888, T>,
    unselected_style: TextStyle<Rgb888, T>
}

impl<'a, T,U, const N:usize> Menu<'a,T,U, N>
where T: Font + Copy + Clone,
      U: Font + Copy + Clone,
{
    pub fn new(menu_title:&'a str, menu_items_str:[&'a str; N], item_font:T, title_font:U, item_spacing:usize) -> Self{
        let background = PrimitiveStyleBuilder::new()
            .fill_color(Rgb888::BLACK)
            .build();
        let foreground = PrimitiveStyleBuilder::new()
            .fill_color(Rgb888::CYAN)
            .build();

        let menu_background = Rectangle::new(Point::new(SCREEN_WIDTH as i32/2 - 100, 20),
                                                   Point::new(SCREEN_WIDTH as i32/2 + 100, SCREEN_HEIGHT as i32 - 20))
            .into_styled(background);

        let menu_foreground = Rectangle::new(Point::new(SCREEN_WIDTH as i32/2 - 100, 20),
                                             Point::new(SCREEN_WIDTH as i32/2 + 100, SCREEN_HEIGHT as i32 - 20))
            .into_styled(foreground);
        let unselected_style = TextStyleBuilder::new(item_font)
            .text_color(Rgb888::WHITE)
            .background_color(Rgb888::CYAN)
            .build();
        let selected_style = TextStyleBuilder::new(item_font)
            .text_color(Rgb888::YELLOW)
            .background_color(Rgb888::CYAN)
            .build();
        let title_style = TextStyleBuilder::new(title_font)
            .background_color(Rgb888::CYAN)
            .text_color(Rgb888::RED)
            .build();
        let menu_title = Text::new(menu_title, Point::new(menu_foreground.top_left().x+5, menu_foreground.top_left().y))
            .into_styled(title_style);

        let mut menu_items = [Text::new("", Point::new(0, 0)).into_styled(unselected_style); N];
        let x_coord = menu_foreground.top_left().x+5;
        let mut y_coord = menu_foreground.top_left().y+item_spacing as i32;
        for i in 0..N{
            menu_items[i] = Text::new(menu_items_str[i], Point::new(x_coord, y_coord))
                .into_styled(unselected_style);
            y_coord += item_spacing as i32;
        }
        menu_items[0].style = selected_style;

        Self { menu_items, menu_title, selected_counter: 0, menu_background,menu_foreground, unselected_style, selected_style}
    }

    /// Displays the menu on the screen
    pub fn show_menu(&mut self, disp: &mut Framebuffer){
        self.menu_foreground.draw(disp);
        self.menu_title.draw(disp);
        for item in &mut self.menu_items{
            item.draw(disp);
        }
    }

    /// Moves the currently highlighted menu item to one lower
    pub fn move_down(&mut self, disp: &mut Framebuffer){
        if self.selected_counter+1 < N{
            self.menu_items[self.selected_counter].style = self.unselected_style;
            self.menu_items[self.selected_counter+1].style = self.selected_style;
            self.menu_items[self.selected_counter].draw(disp);
            self.menu_items[self.selected_counter+1].draw(disp);
            self.selected_counter += 1;
        }
    }
    /// Moves the currently highlighted menu item to one higher
    pub fn move_up(&mut self, disp: &mut Framebuffer){
        if self.selected_counter != 0{
            self.menu_items[self.selected_counter].style = self.unselected_style;
            self.menu_items[self.selected_counter-1].style = self.selected_style;
            self.menu_items[self.selected_counter].draw(disp);
            self.menu_items[self.selected_counter-1].draw(disp);
            self.selected_counter -= 1;
        }
    }

    /// Returns the currently selected menu item's index
    pub fn return_selected_index(&self) -> usize{
        self.selected_counter
    }

    /// Turns the menu to black, effectively hiding it from view. Also resets the selected counter to 0.
    pub fn hide_menu(&mut self, disp: &mut Framebuffer){
        self.menu_background.draw(disp);
        self.menu_items[self.selected_counter].style = self.unselected_style;
        self.menu_items[0].style = self.selected_style;
        self.selected_counter = 0;
    }
}

#[feature(min_const_generics)]
pub struct MenuBuilder<'a, T, U, const N: usize>
where   T: Font + Copy + Clone,
        U: Font + Copy + Clone,
{
    items: Option<[&'a str; N]>,
    title: Option<&'a str>,
    menu_background_color: Option<Rgb888>,
    menu_foreground_color: Option<Rgb888>,
    menu_stroke_color: Option<Rgb888>,
    menu_stroke_width: Option<usize>,
    menu_title_font: Option<T>,
    menu_item_font: Option<U>,
    menu_title_color: Option<Rgb888>,
    menu_item_color: Option<Rgb888>,
    menu_wrapping_enabled: Option<bool>,
}

