use crate::dump::Dumper;
use crate::parser::Parsed;
use crate::ui::List;

use console_engine::{pixel, screen::Screen, Color, ConsoleEngine, KeyCode, KeyModifiers};

pub struct App {
    engine: ConsoleEngine,
    dumper: Dumper,
    list: List,
}

pub enum Action {
    NoAction,
    Action,
    Exit,
}

#[derive(Debug)]
pub enum AppError {
    DumperCreationError,
    ListenError,
}

impl App {
    pub fn new(engine: ConsoleEngine, columns: usize, interface: String) -> Result<Self, AppError> {
        let list_screen = Screen::new(engine.get_width() - 2, engine.get_height() - 2);

        let dumper = if let Some(dumper) = Dumper::new(interface) {
            dumper
        } else {
            return Err(AppError::DumperCreationError);
        };

        let mut list = List::new(columns, list_screen);
        list.spacing[1] = 0.05;
        list.auto_space_rest(columns - 2);

        Ok(App {
            engine,
            dumper,
            list,
        })
    }

    pub fn start(&mut self) -> Result<(), AppError> {
        self.engine.fill(pixel::pxl_bg(' ', Color::Black));

        // Start packet listener
        match self.dumper.start_listening() {
            Ok(_) => {}
            Err(_) => return Err(AppError::ListenError),
        }

        // Draw the initial state of window
        self.engine.clear_screen();
        self.list.draw();
        self.engine.print_screen(1, 1, &self.list.screen);
        self.engine.rect(
            0,
            0,
            self.engine.get_width() as i32 - 1,
            self.engine.get_height() as i32 - 1,
            pixel::pxl('ඞ'),
        );
        self.engine.draw();

        loop {
            self.engine.wait_frame();
            let mut changed = false;

            match self.dumper.update() {
                Ok(a) => {
                    self.add_packets(a);
                    changed = true;
                }
                Err(_) => {}
            }

            match self.handle_input() {
                Action::NoAction => {}
                Action::Action => {
                    changed = true;
                }
                Action::Exit => {
                    break;
                }
            };

            match self.engine.get_resize() {
                Some((x, y)) => self.resize(x, y),
                None => {}
            }

            if changed {
                self.list.draw();
                self.engine.print_screen(1, 1, &self.list.screen);
                self.engine.draw();
            }
        }
        Ok(())
    }

    fn resize(&mut self, x: u16, y: u16) {
        self.engine.resize(x as u32, y as u32);
        self.engine
            .rect(0, 0, x as i32 - 1, y as i32 - 1, pixel::pxl('█'));

        self.list.resize(x, y);
    }

    fn add_packets(&mut self, count: usize) {
        let dumper_length = self.dumper.packets.len();

        for x in dumper_length - count..dumper_length {
            let parsed = Parsed::parse(&self.dumper.packets[x]);
            self.list.add_new(parsed.into());
        }
    }

    // TODO: This is terrible
    pub fn handle_input(&mut self) -> Action {
        let action = if self.engine.is_key_pressed(KeyCode::Char('q')) {
            Action::Exit
        } else if self
            .engine
            .is_key_pressed_with_modifier(KeyCode::Char('J'), KeyModifiers::SHIFT)
        {
            self.list.select_end();
            Action::Action
        } else if self
            .engine
            .is_key_pressed_with_modifier(KeyCode::Char('K'), KeyModifiers::SHIFT)
        {
            self.list.select_start();
            Action::Action
        } else if self.engine.is_key_pressed(KeyCode::Char('j')) {
            self.list.select_next();
            Action::Action
        } else if self.engine.is_key_pressed(KeyCode::Char('k')) {
            self.list.select_prev();
            Action::Action
        } else {
            Action::NoAction
        };

        action
    }
}
