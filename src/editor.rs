use core::cmp::min;
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use std::{
    env::{self},
    io::Error,
};

mod view;
use view::View;
mod terminal;
use terminal::{Position, Size, Terminal};

#[derive(Default, Clone, Copy)]
struct Location {
    x: usize,
    y: usize,
}

#[derive(Default)]
pub struct Editor {
    flag_quit: bool,
    location: Location,
    view: View,
}

impl Editor {
    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        self.handle_args();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn handle_args(&mut self) {
        let args: Vec<String> = env::args().collect();
        if let Some(file_name) = args.get(1) {
            self.view.load(file_name);
        }
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
        let Location { mut x, mut y } = self.location;
        let Size { height, width } = Terminal::size()?;

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

    // fn evaluate_event(&mut self, event: &Event) -> Result<(), Error> {
    //     if let Key(key_event) = event {
    //         match key_event {
    //             KeyEvent {
    //                 code: KeyCode::Char('q'),
    //                 modifiers: KeyModifiers::CONTROL,
    //                 kind: KeyEventKind::Press,
    //                 ..
    //             } => {
    //                 self.flag_quit = true;
    //             }
    //             _ => self.move_point(key_event)?,
    //             Event::Resize(width_u16, height_u16) => {
    //                 #[allow(clippy::as_conversions)]
    //                 let height = height_u16 as usize;
    //                 #[allow(clippy::as_conversions)]
    //                 let width = width_u16 as usize;
    //                 self.view.resize(Size { height, width });
    //             }
    //         }
    //     }
    //
    //     Ok(())
    // }
    //

    fn evaluate_event(&mut self, event: &Event) -> Result<(), Error> {
        match event {
            Event::Key(key_event) => match key_event {
                KeyEvent {
                    code: KeyCode::Char('q'),
                    modifiers: KeyModifiers::CONTROL,
                    kind: KeyEventKind::Press,
                    ..
                } => {
                    self.flag_quit = true;
                }
                _ => {
                    self.move_point(key_event)?;
                }
            },
            Event::Resize(width_u16, height_u16) => {
                let height = *height_u16 as usize;
                let width = *width_u16 as usize;
                self.view.resize(Size { height, width });
            }
            _ => {} //
        }
        Ok(())
    }

    fn refresh_screen(&mut self) -> Result<(), Error> {
        Terminal::hide_caret()?;
        Terminal::move_caret_to(Position::default())?;
        if self.flag_quit {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye!\r\n")?;
        } else {
            self.view.render()?;
            Terminal::move_caret_to(Position {
                column: self.location.x,
                row: self.location.y,
            })?;
        }
        Terminal::show_caret()?;
        Terminal::execute()?;
        Ok(())
    }
}
