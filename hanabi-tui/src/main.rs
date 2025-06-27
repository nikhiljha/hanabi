use hanabi::{Clue, HanabiGame, Player};
use hanabi::cards::{Rank, Suit};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame, Terminal,
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;

struct App {
    game: HanabiGame,
    input: String,
    messages: Vec<String>,
    show_help: bool,
}

impl App {
    fn new() -> App {
        let variant = hanabi::variants::NoVariant;
        let game = HanabiGame::new(vec![
            Player::new("Alice".to_string()),
            Player::new("Bob".to_string()),
        ], variant);
        
        App {
            game,
            input: String::new(),
            messages: vec!["Welcome to Hanabi! Type 'help' for commands.".to_string()],
            show_help: false,
        }
    }

    fn handle_input(&mut self, input: String) -> bool {
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() {
            return false;
        }

        match parts[0] {
            "exit" | "quit" | "q" => return true,
            "help" | "h" => {
                self.show_help = !self.show_help;
                return false;
            }
            "play" => {
                if parts.len() != 2 {
                    self.messages.push("Usage: play <card_index>".to_string());
                    return false;
                }
                match parts[1].parse::<usize>() {
                    Ok(index) => {
                        let action = hanabi::AnnotatedAction {
                            player: self.game.current_player(),
                            action: hanabi::Action::Play(index),
                        };
                        match self.game.act(action) {
                            Ok(()) => self.messages.push("Card played successfully!".to_string()),
                            Err(e) => self.messages.push(format!("Error: {:?}", e)),
                        }
                    }
                    Err(_) => self.messages.push("Invalid card index".to_string()),
                }
            }
            "discard" => {
                if parts.len() != 2 {
                    self.messages.push("Usage: discard <card_index>".to_string());
                    return false;
                }
                match parts[1].parse::<usize>() {
                    Ok(index) => {
                        let action = hanabi::AnnotatedAction {
                            player: self.game.current_player(),
                            action: hanabi::Action::Discard(index),
                        };
                        match self.game.act(action) {
                            Ok(()) => self.messages.push("Card discarded successfully!".to_string()),
                            Err(e) => self.messages.push(format!("Error: {:?}", e)),
                        }
                    }
                    Err(_) => self.messages.push("Invalid card index".to_string()),
                }
            }
            "rank" => {
                if parts.len() != 3 {
                    self.messages.push("Usage: rank <rank> <player_index>".to_string());
                    return false;
                }
                match (Rank::try_from(parts[1]), parts[2].parse::<usize>()) {
                    (Ok(rank), Ok(target)) => {
                        let action = hanabi::AnnotatedAction {
                            player: self.game.current_player(),
                            action: hanabi::Action::Clue {
                                clue: Clue::Rank(rank),
                                target,
                            },
                        };
                        match self.game.act(action) {
                            Ok(()) => self.messages.push("Rank clue given successfully!".to_string()),
                            Err(e) => self.messages.push(format!("Error: {:?}", e)),
                        }
                    }
                    _ => self.messages.push("Invalid rank or player index".to_string()),
                }
            }
            "suit" => {
                if parts.len() != 3 {
                    self.messages.push("Usage: suit <suit> <player_index>".to_string());
                    return false;
                }
                match (Suit::try_from(parts[1]), parts[2].parse::<usize>()) {
                    (Ok(suit), Ok(target)) => {
                        let action = hanabi::AnnotatedAction {
                            player: self.game.current_player(),
                            action: hanabi::Action::Clue {
                                clue: Clue::Suit(suit),
                                target,
                            },
                        };
                        match self.game.act(action) {
                            Ok(()) => self.messages.push("Suit clue given successfully!".to_string()),
                            Err(e) => self.messages.push(format!("Error: {:?}", e)),
                        }
                    }
                    _ => self.messages.push("Invalid suit or player index".to_string()),
                }
            }
            _ => {
                self.messages.push("Unknown command. Type 'help' for available commands.".to_string());
            }
        }

        if self.messages.len() > 10 {
            self.messages.remove(0);
        }

        false
    }

    fn suit_color(suit: &Suit) -> Color {
        match suit {
            Suit::Red => Color::Red,
            Suit::Blue => Color::Blue,
            Suit::Green => Color::Green,
            Suit::Yellow => Color::Yellow,
            Suit::Purple => Color::Magenta,
            Suit::Rainbow => Color::White,
            Suit::Black => Color::DarkGray,
            Suit::Pink => Color::LightMagenta,
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Enter => {
                        let input = app.input.clone();
                        app.input.clear();
                        if app.handle_input(input) {
                            break;
                        }
                    }
                    KeyCode::Char(c) => {
                        app.input.push(c);
                    }
                    KeyCode::Backspace => {
                        app.input.pop();
                    }
                    KeyCode::Esc => {
                        if app.show_help {
                            app.show_help = false;
                        } else {
                            break;
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(())
}

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(3),
            Constraint::Length(5),
        ])
        .split(f.size());

    let title = Paragraph::new("🎴 Hanabi TUI 🎴")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    let game_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);

    let left_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(8), Constraint::Min(5)])
        .split(game_area[0]);

    let score_text = format!("Score: {}", app.game.score());
    let score = Paragraph::new(score_text)
        .style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL).title("Game Status"));
    f.render_widget(score, left_area[0]);

    let mut stack_lines = Vec::new();
    for (suit, cards) in app.game.stacks() {
        let suit_color = App::suit_color(suit);
        let stack_text = if cards.is_empty() {
            format!("{:?}: Empty", suit)
        } else {
            format!("{:?}: {}", suit, cards.len())
        };
        stack_lines.push(Line::from(Span::styled(stack_text, Style::default().fg(suit_color))));
    }
    let stacks = Paragraph::new(stack_lines)
        .block(Block::default().borders(Borders::ALL).title("Stacks"))
        .wrap(Wrap { trim: true });
    f.render_widget(stacks, left_area[1]);

    let mut player_lines = Vec::new();
    for (i, player) in app.game.players().iter().enumerate() {
        let is_current = i == app.game.current_player();
        let player_style = if is_current {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };
        
        let prefix = if is_current { "► " } else { "  " };
        player_lines.push(Line::from(Span::styled(
            format!("{}Player {}: {}", prefix, i, player.name),
            player_style,
        )));
        
        let hand_text: Vec<String> = player.hand.iter().enumerate()
            .map(|(idx, card)| format!("[{}] {}", idx, card.card()))
            .collect();
        player_lines.push(Line::from(Span::styled(
            format!("    {}", hand_text.join(" ")),
            Style::default().fg(Color::Gray),
        )));
    }
    
    let players = Paragraph::new(player_lines)
        .block(Block::default().borders(Borders::ALL).title("Players"))
        .wrap(Wrap { trim: true });
    f.render_widget(players, game_area[1]);

    let input = Paragraph::new(app.input.as_str())
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL).title("Command"));
    f.render_widget(input, chunks[2]);

    let messages: Vec<ListItem> = app
        .messages
        .iter()
        .map(|m| ListItem::new(m.as_str()))
        .collect();
    let messages_list = List::new(messages)
        .block(Block::default().borders(Borders::ALL).title("Messages"))
        .style(Style::default().fg(Color::White));
    f.render_widget(messages_list, chunks[3]);

    if app.show_help {
        let help_area = centered_rect(60, 70, f.size());
        f.render_widget(Clear, help_area);
        
        let help_text = vec![
            Line::from("Commands:"),
            Line::from(""),
            Line::from("play <index>     - Play card at index"),
            Line::from("discard <index>  - Discard card at index"),
            Line::from("rank <rank> <player> - Give rank clue"),
            Line::from("suit <suit> <player> - Give suit clue"),
            Line::from("help / h         - Toggle this help"),
            Line::from("exit / quit / q  - Exit game"),
            Line::from(""),
            Line::from("Press ESC to close help"),
        ];
        
        let help = Paragraph::new(help_text)
            .block(Block::default().borders(Borders::ALL).title("Help"))
            .style(Style::default().fg(Color::Cyan))
            .wrap(Wrap { trim: true });
        f.render_widget(help, help_area);
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
