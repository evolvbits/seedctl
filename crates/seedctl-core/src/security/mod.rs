//! Cold-wallet security warning screen.
//!
//! [`Security::warning`] renders a full-screen scrollable disclaimer that the
//! user must read completely before being allowed to proceed. Once the bottom
//! of the text is reached, the user is prompted to type a required confirmation
//! phrase verbatim. Only an exact match unlocks the next step; pressing
//! `Esc` or `Ctrl+C` at any point exits the process immediately.
//!
//! The screen is rendered in the terminal's **alternate buffer** so the
//! disclaimer does not pollute the scrollback history.

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

/// Renders the cold-wallet security disclaimer and enforces user confirmation.
///
/// Uses the terminal alternate screen and raw mode to display a scrollable
/// full-screen warning. The user must scroll to the end and type the exact
/// `required` confirmation phrase before the call returns successfully.
///
/// Pressing `Esc` or `Ctrl+C` at any point terminates the entire process
/// via [`std::process::exit(0)`].
pub struct Security;

impl Security {
  /// Restores the terminal to its normal state and optionally exits the process.
  ///
  /// Always disables raw mode and leaves the alternate screen. If `success` is
  /// `false`, the process is terminated with exit code `0` (user aborted).
  fn cleanup_and_exit(&self, success: bool) {
    let _ = terminal::disable_raw_mode();
    let _ = execute!(io::stdout(), LeaveAlternateScreen);
    if !success {
      std::process::exit(0);
    }
  }

  /// Displays the full-screen security warning and waits for the user to
  /// confirm by typing `required` exactly.
  ///
  /// # Behaviour
  ///
  /// 1. Enters the alternate screen buffer and enables raw mode.
  /// 2. Renders the [`WARNING_TEXT`] paginated to the current terminal height.
  /// 3. While the end of the text has not been reached, a scroll hint is shown
  ///    at the bottom of the screen (`[V] Scroll down…`).
  /// 4. Once the last line is visible, a confirmation prompt appears below the
  ///    text asking the user to type `required` verbatim.
  /// 5. An incorrect answer clears the input buffer and shows an inline error.
  /// 6. A correct answer calls [`cleanup_and_exit(true)`] and returns `Ok(())`.
  ///
  /// # Errors
  ///
  /// Returns `Err` only if a crossterm I/O operation fails (e.g. the terminal
  /// cannot be put into raw mode). In practice this should never happen on
  /// supported platforms.
  ///
  /// # Panics
  ///
  /// Does not panic; all terminal operations use `?` or `let _ =` to suppress
  /// non-critical failures.
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

      // Reserve extra rows at the bottom for the confirmation prompt when the
      // user has scrolled to the end; otherwise reserve only the scroll hint.
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

      // ── Render the visible portion of the warning text ──────────────────
      for i in 0..display_height {
        if let Some(line) = lines.get(current_line + i) {
          // Clamp to terminal width to avoid line wrapping artefacts.
          let line_to_print = if line.len() > term_width as usize {
            &line[..term_width as usize]
          } else {
            line
          };
          write!(stdout, "{}\r\n", line_to_print)?;
        }
      }

      let is_at_end = current_line + display_height >= lines.len();

      // ── Dynamic bottom UI ────────────────────────────────────────────────
      if is_at_end {
        // Blank line separating text from the prompt.
        write!(stdout, "\r\n")?;

        // Confirmation question.
        write!(
          stdout,
          "{} Type EXACTLY: {}\r\n",
          "?".cyan().bold(),
          required.bold()
        )?;

        // Input line.
        write!(stdout, "{} {}", ">".cyan(), input_buffer)?;

        // Inline error message displayed below the input on a wrong attempt.
        if error_msg {
          write!(
            stdout,
            "\r\n{} {}",
            "x".red().bold(),
            "Incorrect phrase.".red()
          )?;
          // Move cursor back to the input line so the user can keep typing.
          execute!(
            stdout,
            cursor::MoveUp(1),
            cursor::MoveTo(2 + input_buffer.len() as u16, (display_height + 2) as u16)
          )?;
        }
      } else {
        // Scroll hint fixed at the very bottom of the window.
        execute!(stdout, cursor::MoveTo(0, term_height.saturating_sub(1)))?;
        write!(
          stdout,
          "\x1b[7m [V] Scroll down to confirm | ESC/Ctrl+C to Abort \x1b[0m"
        )?;
      }
      stdout.flush()?;

      // ── Key event handling ───────────────────────────────────────────────
      if let Event::Key(key_event) = event::read()? {
        // Ignore key-release and key-repeat events; only process key-press.
        if key_event.kind != KeyEventKind::Press {
          continue;
        }

        // Global abort: Esc or Ctrl+C at any scroll position.
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
              // The first Enter that arrives when the end is newly reached is
              // consumed without submitting, to avoid accidentally confirming.
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
