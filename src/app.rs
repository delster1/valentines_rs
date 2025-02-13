use crate::ui::ui;
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    symbols::Marker,
    widgets::{
        canvas::{Canvas, Circle, Painter, Shape},
        Block, Borders, Clear, List, ListItem, Paragraph, Widget, Wrap,
    },
    DefaultTerminal,
};
use std::{
    borrow::BorrowMut,
    fmt, io,
    time::{Duration, Instant},
};
#[derive(Debug, Default, Clone)]
pub struct Heart {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
    pub color: Color,
}

impl Shape for Heart {
    fn draw(&self, painter: &mut Painter<'_, '_>) {
        // Iterate over the parameter t to draw the heart
        for t in (0..=360).map(|i| i as f64 * (std::f64::consts::PI / 180.0)) {
            let heart_x = 16.0 * t.sin().powi(3);
            let heart_y =
                13.0 * t.cos() - 5.0 * (2.0 * t).cos() - 2.0 * (3.0 * t).cos() - (4.0 * t).cos();

            // Scale and translate the heart
            let x = self.radius.mul_add(heart_x, self.x);
            let y = self.radius.mul_add(heart_y, self.y); // Negate y to flip it upright

            // Paint the point if it's within the canvas bounds
            if let Some((x, y)) = painter.get_point(x, y) {
                painter.paint(x, y, self.color);
            }
        }
    }
}
#[derive(Debug, Default)]
pub struct App<'a> {
    pub title: &'a str,
    pub exit: bool,
    pub current_screen: CurrentScreen,
    pub intro_screen: IntroScreenOptions,
    heart: Heart,
    pub playground: Rect,
    vx: f64,
    vy: f64,
    tick_count: u64,
    pub offset: u8,
    marker: Marker,
}

pub enum CurrentScreen {
    IntroScreen,
    WillYouBeMyValentine,
    NotValentine,
    YesValentine,
    DateInfo,
}

pub enum IntroScreenOptions {
    Screen1, // as you might know, there's a question i've been meaning to ask
    Screen2, // i'm sorry it took longer than i expected, this took a while!
    Screen3, // You've waited long enough my pretty lady, I have a question...
}

impl fmt::Debug for IntroScreenOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Default for IntroScreenOptions {
    fn default() -> Self {
        IntroScreenOptions::Screen1
    }
}

impl fmt::Debug for CurrentScreen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl Default for CurrentScreen {
    fn default() -> Self {
        CurrentScreen::WillYouBeMyValentine
    }
}

impl<'a> App<'a> {
    pub fn new() -> Self {
        Self {
            title: "To My Valentie",
            exit: false,
            current_screen: CurrentScreen::IntroScreen,
            intro_screen : IntroScreenOptions::Screen1,
            heart: Heart {
                x: 5.0,
                y: 5.0,
                radius: 0.5,
                color: Color::LightMagenta,
            },
            playground: Rect::new(0, 0, 200, 100),
            vx: 3.0,
            vy: 3.0,
            offset: 0,
            tick_count: 0,
            marker: Marker::Braille,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let tick_rate = Duration::from_millis(16);
        let mut last_tick = Instant::now();
        while !self.exit {
            terminal.draw(|frame| ui(frame, self.borrow_mut()))?;

            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key_event) = event::read()? {
                    self.handle_key_event(key_event).unwrap();
                }
            }
            if last_tick.elapsed() >= tick_rate {
                self.on_tick();
                last_tick = Instant::now();
            }
        }
        Ok(())
    }

    fn on_tick(&mut self) {
        self.tick_count += 1;
        
        if (self.tick_count % 12) == 0 {
            self.offset = (self.offset + 1) % 4;
            // println!("{}",self.offset);
        }
        let ball = self.heart.clone();
        // println!("x&y: {},{}", ball.x, ball.y);
        let playground = self.playground;
        if ball.x - ball.radius < f64::from(playground.left())
            || ball.x + ball.radius > f64::from(playground.right())
        {
            self.vx = -self.vx;
        }
        if ball.y - ball.radius < f64::from(playground.top())
            || ball.y + ball.radius > f64::from(playground.bottom())
        {
            self.vy = -self.vy;
        }

        self.heart.x += self.vx;
        self.heart.y += self.vy;
    }
    fn exit(&mut self) {
        self.exit = true;
    }
    pub fn pong_canvas(&self, area: Rect) -> impl Widget + '_ {
        Canvas::default()
            .background_color(Color::Black)
            .block(Block::bordered().title("I love you!!!"))
            .marker(self.marker)
            .paint(|ctx| {
                ctx.draw(&self.heart);
            })
            .x_bounds([0.0, f64::from(area.width)])
            .y_bounds([0.0, f64::from(area.height)])
    }
}

impl<'a> App<'a> {
    fn handle_key_event(&mut self, key_event: KeyEvent) -> io::Result<()> {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            _ => {
                match self.current_screen {
                    CurrentScreen::IntroScreen => {
                        match self.intro_screen  {
                            IntroScreenOptions::Screen1 => {
                                self.intro_screen = IntroScreenOptions::Screen2;
                            },
                            IntroScreenOptions::Screen2 => {
                                self.intro_screen = IntroScreenOptions::Screen3;
                            },
                            IntroScreenOptions::Screen3 => {
                                self.intro_screen = IntroScreenOptions::Screen3;
                                self.current_screen = CurrentScreen::WillYouBeMyValentine;
                                self.heart.x = 10.0;
                                self.heart.y = 10.0;
                            }
                        }
                    },
                    CurrentScreen::WillYouBeMyValentine => {
                        self.handle_wybmv_event(key_event);
                    },
                    CurrentScreen::NotValentine => {
                        self.handle_notvalentine_event(key_event);
                                self.heart.x = 10.0;
                                self.heart.y = 10.0;
                    },
                    CurrentScreen::YesValentine => {
                        self.handle_yesvalentine_event(key_event);
                                self.heart.x = 10.0;
                                self.heart.y = 10.0;
                    },
                    CurrentScreen::DateInfo => {
                    }

                }
            }
                
        }
        Ok(())
    }
    fn handle_wybmv_event(&mut self, key_event : KeyEvent) {
        match key_event.code {
            KeyCode::Char('y') | KeyCode::Char('Y') => {
                self.current_screen = CurrentScreen::YesValentine
            },
            KeyCode::Char('n') | KeyCode::Char('N') => {
                self.current_screen = CurrentScreen::NotValentine
            }
            _ => {}
                
        }

    }
    fn handle_notvalentine_event(&mut self, key_event : KeyEvent) {
        match key_event.code {
            _ => {
                self.current_screen = CurrentScreen::WillYouBeMyValentine
            }
            _ => {}
                
        }

    }
    fn handle_yesvalentine_event(&mut self, key_event : KeyEvent) {
        match key_event.code {
            KeyCode::Char('y') | KeyCode::Char('Y') => {
                self.current_screen = CurrentScreen::DateInfo
            },
            KeyCode::Char('n') | KeyCode::Char('N') => {
                self.current_screen = CurrentScreen::WillYouBeMyValentine
            }
            _ => {}

        }
    }
}
