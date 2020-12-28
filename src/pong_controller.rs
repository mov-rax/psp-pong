mod paddle;
mod ball;
mod opponent;
mod menu;

pub mod pong_controller {
    use crate::pong_controller::paddle::{Paddle, PaddleBuilder, PaddleMovementSpeed};
    use crate::pong_controller::opponent::{Opponent, OpponentDifficulty};
    use crate::pong_controller::ball::{Ball, BallAxes};
    use psp::sys::{SceCtrlData, CtrlButtons};
    use psp::sys::{sceRtcGetCurrentTick as getTick, sceCtrlReadBufferPositive as getInput};
    use psp::embedded_graphics::Framebuffer;
    use embedded_graphics::style::{TextStyleBuilder, PrimitiveStyleBuilder};
    use embedded_graphics::fonts::{Font6x12, Text, Font8x16, Font12x16};
    use embedded_graphics::pixelcolor::Rgb888;
    use embedded_graphics::prelude::{RgbColor, Point};
    use embedded_graphics::drawable::Drawable;
    use arrayvec::ArrayString;
    use core::fmt::Write;
    use rand_chacha::{ChaCha20Rng, ChaChaRng};
    use rand::SeedableRng;
    use crate::pong_controller::menu::Menu;

    struct Score {
        player: u32,
        opponent: u32,
    }

    pub struct PongController{
        player: Paddle,
        opponent: Opponent,
        ball: Ball,
        score: Score,
        ticks_per_update: u32,
        display: Framebuffer,
        rng: ChaCha20Rng, // A random number generator that initialized at runtime
    }

    impl PongController{
        pub fn new(ticks_per_second: u32) -> Self{
            let player = PaddleBuilder::new()
                .set_default_dimensions()
                .set_speed(PaddleMovementSpeed::Normal)
                .set_x(10)
                .set_y(psp::SCREEN_HEIGHT as i32/2)
                .build();

            let mut seed = 0;
            unsafe { getTick(&mut seed)}; // the PSP's clock tick will be utilized for creating the seed for initializing the rng
            let mut rng = ChaChaRng::seed_from_u64(seed);

            let opponent = Opponent::new(OpponentDifficulty::Hard);
            let ball = Ball::new(psp::SCREEN_WIDTH as i32/2, psp::SCREEN_HEIGHT as i32/2, 10, 3, &mut rng);
            let score = Score {player: 0, opponent: 0};
            let ticks_per_update = 1000/ticks_per_second; //gets the amount of time between screen updates



            Self {player, opponent, ball, score, ticks_per_update , display: Framebuffer::new(), rng}
        }

        pub fn run(&mut self){
            let score_style = TextStyleBuilder::new(Font6x12)
                .background_color(Rgb888::BLACK)
                .text_color(Rgb888::WHITE)
                .build();


            let mut last_tick = 0;
            let mut current_tick = 0;
            let mut input = SceCtrlData::default();
            let mut player_score_buf = ArrayString::<[_;12]>::new();
            let mut opponent_score_buf = ArrayString::<[_;12]>::new();

            let mut player_score = Text::new("0", Point::new(5, 5))
                .into_styled(score_style);
            let mut opponent_score = Text::new("0", Point::new(psp::SCREEN_WIDTH as i32 - 50, 5))
                .into_styled(score_style);

            let mut main_menu = Menu::new("Main Menu", ["Set Difficulty", "Reset Score", "Close Menu"], Font12x16, Font12x16,17 );
            let mut difficulty_menu = Menu::new("Difficulty Settings", ["Very Easy", "Easy", "Normal", "Hard", "Very Hard"], Font12x16, Font12x16, 17);
            let mut menu_f = false; // flag to check if the menu should be open, pausing all other events
            let mut menu_df = false; // flag to check if down on the d-pad was pressed.
            let mut menu_uf = false; // flag to check if up on the d-pad was pressed.
            let mut menu_sf = false; // flag to check that x was pressed.

            unsafe { getTick(&mut last_tick) };
            loop {

                unsafe {
                    getTick(&mut current_tick); //updates the current tick
                    getInput(&mut input, 1); // updates the buttons currently being pressed
                }

                if current_tick > last_tick + self.ticks_per_update as u64 { // Time to update the screen :)

                    if menu_f{ // If the menu has been enabled

                        if !input.buttons.contains(CtrlButtons::DOWN){
                            menu_df = false;
                        }

                        if !input.buttons.contains(CtrlButtons::UP){
                            menu_uf = false;
                        }

                        if !input.buttons.contains(CtrlButtons::CROSS){
                            menu_sf = false;
                        }

                        if input.buttons.contains(CtrlButtons::DOWN) && !menu_df{
                            main_menu.move_down(&mut self.display);
                            menu_df = true;
                        }

                        if input.buttons.contains(CtrlButtons::UP) && !menu_uf{
                            main_menu.move_up(&mut self.display);
                            menu_uf = true;
                        }

                        if input.buttons.contains(CtrlButtons::CROSS) && !menu_sf{
                            match main_menu.return_selected_index(){
                                0 => { // Set difficulty

                                },
                                1 => { // Reset Score
                                    menu_f = false;
                                    menu_uf = false;
                                    menu_df = false;
                                    menu_sf = false;
                                    self.score.player = 0;
                                    self.score.opponent = 0;
                                    main_menu.hide_menu(&mut self.display);
                                },
                                2 => { // Close Menu
                                    menu_f = false;
                                    menu_uf = false;
                                    menu_df = false;
                                    menu_sf = false;
                                    main_menu.hide_menu(&mut self.display);
                                },
                                n => {
                                    psp::dprintln!("An error has occurred while selecting something in the main menu");
                                    psp::dprintln!("{} is not a valid index", n);
                                    panic!("{} is not a valid index", n);
                                }
                            }

                        }

                        continue; // All other events will not execute as long as the window is not closed
                    }

                    if input.buttons.contains(CtrlButtons::START){ // start button is pressed. Menu should appear (maybe)
                        menu_f = true;
                        main_menu.show_menu(&mut self.display);
                        continue;
                    }

                    // Time to check the ball
                    if self.ball.get_bounds().top <= 0{
                        self.ball.flip_direction(BallAxes::Vertical); // Makes the ball bounce off the top of the screen
                    } else if self.ball.get_bounds().bottom >= psp::SCREEN_HEIGHT as i32{
                        self.ball.flip_direction(BallAxes::Vertical); // Makes the ball bounce off the bottom of the screen
                    } else if self.player.contains(self.ball.get_bounds().left, self.ball.get_bounds().bottom, true) ||
                        self.player.contains(self.ball.get_bounds().left, self.ball.get_bounds().top, true){ // Check to see if the ball hit the player's paddle
                        self.ball.flip_direction(BallAxes::Horizontal); // Makes the ball bounce off the player's paddle
                        self.player.move_up(&mut self.display);
                        self.player.move_down(&mut self.display);
                    } else if self.opponent.contains(self.ball.get_bounds().right, self.ball.get_bounds().bottom) ||
                        self.opponent.contains(self.ball.get_bounds().right, self.ball.get_bounds().top){ // Check to see if the ball hit the opponent's paddle
                        self.ball.flip_direction(BallAxes::Horizontal); // Makes the ball bounce of the opponent's paddle
                    } else if self.ball.get_bounds().left <= 0{ // Check to see if the ball is in the player's goal
                        self.score.opponent += 1;
                        self.ball.blacken(&mut self.display);
                        self.ball = Ball::new(psp::SCREEN_WIDTH as i32/2, psp::SCREEN_HEIGHT as i32/2, 10, 3, &mut self.rng);
                        opponent_score_buf.clear();
                        write!(&mut opponent_score_buf, "{}", self.score.opponent);

                        opponent_score = Text::new(&opponent_score_buf, Point::new(psp::SCREEN_WIDTH as i32 - 50, 5))
                            .into_styled(score_style); // updates the opponent's score;

                        //Self::wait(1000); //wait for a second before moving the ball again
                    } else if self.ball.get_bounds().right >= psp::SCREEN_WIDTH as i32{ // Check to see if the ball is in the opponent's goal
                        self.score.player += 1;
                        self.ball.blacken(&mut self.display);
                        self.ball = Ball::new(psp::SCREEN_WIDTH as i32/2, psp::SCREEN_HEIGHT as i32/2, 10, 3, &mut self.rng);
                        player_score_buf.clear();
                        write!(&mut player_score_buf, "{}", self.score.player);

                        player_score = Text::new(&player_score_buf, Point::new(5, 5))
                            .into_styled(score_style); // updates the player's score;

                        //Self::wait(1000); //wait for a second before moving the ball again
                    }


                    self.ball.step_direction(&mut self.display); // Move the ball in the direction it is heading.

                    if input.buttons.contains(CtrlButtons::UP) && self.player.get_bounds().top > 0{
                        self.player.move_up( &mut self.display)
                    } else if input.buttons.contains(CtrlButtons::DOWN) && self.player.get_bounds().bottom < psp::SCREEN_HEIGHT as i32{
                        self.player.move_down(&mut self.display)
                    }
                    self.opponent.step_opponent(self.ball.get_bounds(), &mut self.display);
                    player_score.draw(&mut self.display); // draw the player's score
                    opponent_score.draw(&mut self.display); //draw the opponent's score
                }

            }
        }

        /// Suspend operations for N milliseconds
        fn wait(ms: u32){
            let mut last = 0;
            unsafe {
                getTick(&mut last);
            }
            let until = last + ms as u64;

            while last <= until{
                unsafe {let _ = getTick(&mut last);}
            }

        }


    }
}