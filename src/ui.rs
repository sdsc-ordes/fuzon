use crate::{Term, TermMatcher};
use std::io::stdout;

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    Terminal,
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

// Main interaction loop listening to keys, running the search and rendering the UI on each key
// stroke.
pub fn interactive(matcher: &TermMatcher, top_n: Option<usize>) -> Result<()> {

    // Raw mode does not react to SIGINT, hence we capture it below
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut query = String::new();

    loop {
        terminal.draw(|f| {
            draw_ui(f, &query, &matcher, top_n);
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                // Exit on Ctrl-C
                KeyCode::Char(c) if key.modifiers.contains(KeyModifiers::CONTROL) && c == 'c' => break,
                KeyCode::Char(c) => query.push(c),
                KeyCode::Backspace => { query.pop(); },
                KeyCode::Esc => break,
                _ => {}
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}


// Draws the TUI elements
pub fn draw_ui(
    f: &mut Frame, query: &str, matcher: &TermMatcher, top_n: Option<usize>
) {
    // Split the frame into vertical sections
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(1)].as_ref())
        .split(f.area());

    // Input block (displays what the user it typing)
    let input = Paragraph::new(query)
        .block(Block::default().borders(Borders::ALL).title("Query"));

    // Only show hits
    let results = search(matcher, query, top_n)
        .into_iter()
        .filter(|(_, score)| *score > 0.0).collect::<Vec<(&Term, f64)>>();

    // Results block, shows search results
    let items: Vec<ListItem> = results.iter().map(|(term, score)| {
        ListItem::new(format!("[{}] {}", score, term))
    }).collect();
    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Results"))
        .style(Style::default().fg(Color::White));
        
    f.render_widget(input, chunks[0]);
    f.render_widget(list, chunks[1]);

}

// Helper to run the fuzzy search and filter top hits if requested.
pub fn search<'a>(matcher: &'a TermMatcher, query: &str, top_n: Option<usize>) -> Vec<(&'a Term, f64)> {
    let mut results = matcher.rank_terms(query);
    if let Some(top_n) = top_n {
        let take_n = top_n.min(results.len());
        results = results[..take_n].to_vec();
    }
    return results
}
