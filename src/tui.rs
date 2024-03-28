use std::io;
use std::io::stdout;

use crossterm::event;
use crossterm::event::Event;
use crossterm::event::KeyCode;
use crossterm::terminal::disable_raw_mode;
use crossterm::terminal::enable_raw_mode;
use crossterm::terminal::EnterAlternateScreen;
use crossterm::terminal::LeaveAlternateScreen;
use crossterm::ExecutableCommand;
use ratatui::prelude::*;
use ratatui::widgets::*;

use crate::thread::Catalog;
use crate::thread::Thread;

// lazily copied from coggers

pub trait Menu {
    /// Responsible for the `ratatui` loop, and controlled by an event handler.
    /// The event handler is currently implemented globally; this will
    /// probably become a trait impl.
    fn menu(&mut self) -> io::Result<()> {
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;
        let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

        let mut should_quit = false;
        while !should_quit {
            terminal.draw(|frame| Self::render(self, frame))?;
            should_quit = self.get_new_state()?;
        }

        disable_raw_mode()?;
        stdout().execute(LeaveAlternateScreen)?;
        Ok(())
    }

    /// No expensive operations (e.g. network requests) should ever be made
    /// here.
    fn get_new_state(&mut self) -> io::Result<bool>;

    /// Responsible for rendering a single 'frame' in `menu`. Implementation
    /// will vary depending on the data structure, and the intended widget to be
    /// rendered.
    fn render(
        &mut self,
        frame: &mut Frame,
    );
}

impl Menu for Catalog {
    fn render(
        &mut self,
        frame: &mut ratatui::prelude::Frame,
    ) {
        let list = List::new(
            self.posts
                .iter()
                .map(|p| p.sub.as_ref().unwrap().to_string()),
        );
        // let block = Block::default().borders(Borders::TOP);

        frame.render_stateful_widget(
            //
            list, //.block(block),
            frame.size(),
            &mut self.state,
        );

        // https://docs.rs/ratatui/0.26.1/src/demo2/tabs/email.rs.html#97
    }

    fn get_new_state(&mut self) -> io::Result<bool> {
        let noquit = io::Result::Ok(false);

        if !event::poll(std::time::Duration::from_millis(50))? {
            return noquit;
        };
        if let Event::Key(key) = event::read()? {
            if key.kind != event::KeyEventKind::Press {
                return noquit;
            }
            match key.code {
                KeyCode::Char('q') => return Ok(true),
                KeyCode::Char('x') => return Ok(true),
                KeyCode::Char('l') => {
                    let p = self.posts.get(self.state.offset()).unwrap();
                    let t = Thread::new(self.board.to_string(), p.no).unwrap();
                    t.page().unwrap();

                    return noquit;
                }
                KeyCode::Char('j') => {
                    if self.state.offset() < self.posts.len() {
                        *self.state.offset_mut() += 1;
                    }
                    return noquit;
                }
                KeyCode::Char('k') => {
                    if self.state.offset() > 0 {
                        *self.state.offset_mut() -= 1;
                    }
                    return noquit;
                }
                _ => (),
            }
        }

        noquit
    }
}
