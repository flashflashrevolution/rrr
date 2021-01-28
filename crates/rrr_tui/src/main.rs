// use crossterm::{
//     event::{self, DisableMouseCapture, EnableMouseCapture, Event as CEvent, KeyCode},
//     execute,
//     terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
// };

use std::error::Error;

// use std::{
//     error::Error,
//     io::stdout,
//     sync::mpsc,
//     thread,
//     time::{Duration, Instant},
// };

// use tui::{
//     backend::Backend,
//     backend::CrosstermBackend,
//     layout::{Constraint, Direction, Rect},
//     style::{Color, Modifier, Style},
//     text::{Span, Spans},
//     widgets::{Block, Borders, Paragraph, Wrap},
//     Frame, Terminal,
// };

// fn draw_footer<B>(frame: &mut Frame<B>, area: Rect)
// where
//     B: Backend,
// {
//     let text = vec![Spans::from(
//         "This is a paragraph with several lines. You can change style your text the way you want",
//     )];

//     let block = Block::default().borders(Borders::ALL).title(Span::styled(
//         "Footer",
//         Style::default()
//             .fg(Color::Magenta)
//             .add_modifier(Modifier::BOLD),
//     ));

//     Paragraph::new(text).block(block).wrap(Wrap { trim: true });
// }

fn main() -> Result<(), Box<dyn Error>> {
    Ok(())
}
