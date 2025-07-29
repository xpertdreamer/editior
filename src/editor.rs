use std::io::Error;
use crossterm::event::{read, Event::{self, Key}, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use core::cmp::min;

mod terminal;
use terminal::{Terminal, Size, Position};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default, Clone, Copy)]
struct Location {
    x: u16,
    y: u16,
}

#[derive(Default)]
pub struct Editor {
    flag_quit: bool,
    location: Location,
}

impl Editor {
    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn repl(&mut self) -> Result<(), Error> {
        loop {
            self.refresh_screen()?;
            if self.flag_quit {
                break;
            }
            let event = read()?;
            self.evaluate_event(&event)?;
        }
        Ok(())
    }

    fn move_point(&mut self, key_event: &KeyEvent) -> Result<(), Error> {
        let Location {mut x, mut y} = self.location;
        let Size {height, width} = Terminal::size()?;
    
        match key_event {
            KeyEvent {
                code: KeyCode::Up,
                modifiers: KeyModifiers::CONTROL,
                kind: KeyEventKind::Press,
                ..
            } => {
                y = 0;
            }
            KeyEvent {
                code: KeyCode::Down,
                modifiers: KeyModifiers::CONTROL,
                kind: KeyEventKind::Press,
                ..
            } => {
                y = height.saturating_sub(1);
            }
            KeyEvent {
                code: KeyCode::Left,
                modifiers: KeyModifiers::CONTROL,
                kind: KeyEventKind::Press,
                ..
            } => {
                x = 0;
            }
            KeyEvent {
                code: KeyCode::Right,
                modifiers: KeyModifiers::CONTROL,
                kind: KeyEventKind::Press,
                ..
            } => {
                x = width.saturating_sub(1);
            }
        
            KeyEvent { 
                code: KeyCode::Up, 
                kind: KeyEventKind::Press, 
                .. 
            } => {
                y = y.saturating_sub(1);
            }
            KeyEvent { 
                code: KeyCode::Down,
                kind: KeyEventKind::Press,
                .. 
            } => {
                y = min(height.saturating_sub(1), y.saturating_add(1));
            }
            KeyEvent { 
                code: KeyCode::Left,
                kind: KeyEventKind::Press,
                .. 
            } => {
                x = x.saturating_sub(1);
            }
            KeyEvent { 
                code: KeyCode::Right,
                kind: KeyEventKind::Press,
                .. 
            } => {
                x = min(width.saturating_sub(1), x.saturating_add(1));
            }
            _ => (),
        }
    
        self.location = Location { x, y };
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) -> Result<(), Error> {
        if let Key(key_event) = event {
            match key_event {
                KeyEvent {
                    code: KeyCode::Char('q'),
                    modifiers: KeyModifiers::CONTROL, 
                    kind: KeyEventKind::Press,
                    ..
                } => {
                    self.flag_quit = true;
                }
                _ => {self.move_point(key_event)?},
            }
        }

        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::hide_caret()?;
        Terminal::move_caret_to(Position::default())?;
        if self.flag_quit {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye!\r\n")?;
        } else {
            Self::draw_rows()?;
            Terminal::move_caret_to(Position {
                column: self.location.x,
                row: self.location.y,
            })?;
        }
        Terminal::show_caret()?;
        Terminal::execute()?;
        Ok(())
    }

    fn draw_empty_row() -> Result<(), Error> {
        Terminal::print("~")?;
        Ok(())
    }

    fn draw_rows() -> Result<(), Error> {
        let Size {height, ..} = Terminal::size()?;
        for current_row in 0..height {
            Terminal::clear_line()?;
            if current_row == height / 2 {
                Self::draw_welcome_msg()?;
            } else {
                Self::draw_empty_row()?;
            }
            if current_row + 1 < height {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }


    fn draw_welcome_msg() -> Result<(), Error> {
        let mut welcome_msg = format!("{NAME} - version {VERSION}");
        let width = Terminal::size()?.width as usize;
        let length = welcome_msg.len();
        let padding = (width - length) / 2;
        let spaces = " ".repeat(padding - 1);
        welcome_msg = format!("~{spaces}{welcome_msg}");
        welcome_msg.truncate(width);
        Terminal::print(welcome_msg)?;
        Ok(())
    }
}