mod constants;

use crate::security::constants::WARNING_TEXT;
use crossterm::event::KeyEventKind;
use crossterm::{
  cursor, event,
  event::{Event, KeyCode, KeyModifiers},
  execute,
  style::Stylize,
  terminal,
  terminal::{ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{self, Write};

pub struct Security;

impl Security {
  fn cleanup_and_exit(&self, success: bool) {
    let _ = terminal::disable_raw_mode();
    let _ = execute!(io::stdout(), LeaveAlternateScreen);
    if !success {
      std::process::exit(0);
    }
  }

  pub fn warning(&self, required: &str) -> io::Result<()> {
    let lines: Vec<&str> = WARNING_TEXT.lines().collect();
    let mut stdout = io::stdout();
    let mut input_buffer = String::new();
    let mut error_msg = false;
    let mut just_reached_end = false;

    execute!(stdout, EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;

    let mut current_line = 0;

    loop {
      let (term_width, term_height) = terminal::size()?;

      // --- SPACE CONFIGURATION ---
      // If it's at the end, we reserve 4 lines for the prompt (Error, Question, >, space).
      // Otherwise, we'll reserve just 1 for the scroll bar.
      let reserved_rows = if current_line + (term_height as usize) >= lines.len() {
        5
      } else {
        1
      };
      let display_height = (term_height as usize).saturating_sub(reserved_rows);

      execute!(
        stdout,
        terminal::Clear(ClearType::All),
        cursor::MoveTo(0, 0)
      )?;

      // 1. Draw the Text
      for i in 0..display_height {
        if let Some(line) = lines.get(current_line + i) {
          let line_to_print = if line.len() > term_width as usize {
            &line[..term_width as usize]
          } else {
            line
          };
          write!(stdout, "{}\r\n", line_to_print)?;
        }
      }

      let is_at_end = current_line + display_height >= lines.len();

      // 2. Dynamic Interface (Pasted into the text or background)
      if is_at_end {
        // If you've reached the end, the prompt appears just below the last line of the loop above.
        // We use `println!` to ensure it starts on the next line.
        write!(stdout, "\r\n")?;

        // Question (?)
        write!(
          stdout,
          "{} Type EXACTLY: {}\r\n",
          "?".cyan().bold(),
          required.bold()
        )?;

        // Input (>)
        write!(stdout, "{} {}", ">".cyan(), input_buffer)?;

        // Error message (below the input, as in your screenshot)
        if error_msg {
          write!(
            stdout,
            "\r\n{} {}",
            "x".red().bold(),
            "Incorrect phrase.".red()
          )?;
          // We move the cursor back to the line with the ">" to continue typing.
          execute!(
            stdout,
            cursor::MoveUp(1),
            cursor::MoveTo(2 + input_buffer.len() as u16, (display_height + 2) as u16)
          )?;
        }
      } else {
        // While scrolling, the bar remains fixed at the BOTTOM of the window.
        execute!(stdout, cursor::MoveTo(0, term_height.saturating_sub(1)))?;
        write!(
          stdout,
          "\x1b[7m [V] Scroll down to confirm | ESC/Ctrl+C to Abort \x1b[0m"
        )?;
      }
      stdout.flush()?;

      // 3. Keys
      if let Event::Key(key_event) = event::read()? {
        if key_event.kind != KeyEventKind::Press {
          continue;
        }

        if (key_event.code == KeyCode::Char('c')
          && key_event.modifiers.contains(KeyModifiers::CONTROL))
          || key_event.code == KeyCode::Esc
        {
          self.cleanup_and_exit(false);
        }

        if is_at_end {
          match key_event.code {
            KeyCode::Char(c) => {
              error_msg = false;
              input_buffer.push(c);
              just_reached_end = false;
            }
            KeyCode::Backspace => {
              input_buffer.pop();
            }
            KeyCode::Enter => {
              if !just_reached_end && !input_buffer.is_empty() {
                if input_buffer == required {
                  break;
                } else {
                  input_buffer.clear();
                  error_msg = true;
                }
              }
              just_reached_end = false;
            }
            KeyCode::Up => {
              current_line = current_line.saturating_sub(1);
              input_buffer.clear();
              error_msg = false;
              just_reached_end = false;
            }
            _ => {}
          }
        } else {
          match key_event.code {
            KeyCode::Enter | KeyCode::Down | KeyCode::Char(' ') => {
              current_line += 1;
              if current_line + display_height >= lines.len() {
                just_reached_end = true;
              }
            }
            KeyCode::Up => {
              current_line = current_line.saturating_sub(1);
            }
            _ => {}
          }
        }
      }
    }

    self.cleanup_and_exit(true);
    Ok(())
  }
}
