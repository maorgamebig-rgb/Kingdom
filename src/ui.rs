use std::io;
use std::io::{Write, stdout};
use crossterm;
use crossterm::{QueueableCommand, execute};
use crossterm::terminal::{size, Clear, ClearType, SetSize, SetTitle, disable_raw_mode, enable_raw_mode};
use ratatui;
use ratatui::layout::{Constraint, Layout, Spacing};
use ratatui::symbols::merge::MergeStrategy;
use ratatui::widgets::{Block, Paragraph, List, ListItem, ListDirection};
use ratatui::{DefaultTerminal, Frame};
use ratatui::layout::Direction::{Horizontal, Vertical};
use ratatui::style::Style;
use crate::world::GameState;

pub fn ui(game_state: GameState) -> std::io::Result<()> {

    let (og_cols, og_rows) = size()?;
    execute!(
        io::stdout(),
        SetSize(100, 30),
        SetTitle("Kingdom by 2 dumbass"),
        Clear(ClearType::All),
    )?;


    enable_raw_mode()?;


    let mut terminal = ratatui::init();

    let result = app(&mut terminal, &game_state);

    ratatui::restore();


    disable_raw_mode()?;


    execute!(
        io::stdout(),
        SetSize(og_cols, og_rows),
    )?;


    result
}

fn app(terminal: &mut DefaultTerminal, game_state: &GameState) -> std::io::Result<()> {
    loop {
        terminal.draw(|frame| {
            draw_ui(frame, game_state)
        })?;
        // terminal.draw(draw_layout_overlap)?;

        if crossterm::event::read()?.is_key_press() {
            break Ok(());
        }
    }
}

fn draw_ui(frame: &mut Frame, game_state: &GameState) {
    let [title_area, main_area] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Fill(1)
    ])
        .areas(frame.area());

    let title = Paragraph::new("KINGDOM by 2 Dumbasses").centered();

    frame.render_widget(title, title_area);

    draw_layout_merge(frame, main_area, game_state);
}


fn draw_layout_merge(frame: &mut Frame, area: ratatui::layout::Rect, game_state: &GameState) {
    // create a layout that splits the screen into 2 equal columns and the right column into 2 equal rows

    let terminal = Layout::default()
        .direction(Horizontal)
        .constraints([
            Constraint::Percentage(40),
            Constraint::Percentage(60)
        ])
        .spacing(Spacing::Overlap(1))
        .margin(1)
        .split(area);

    let left = Layout::default()
        .direction(Vertical)
        .constraints([
            Constraint::Percentage(60),
            Constraint::Percentage(40)
        ])
        .split(terminal[0]);


    let right_block = Block::bordered()
        .title("Right block")
        .merge_borders(MergeStrategy::Exact);

    let top_left_block = Block::bordered()
        .title("Top Left Block")
        .merge_borders(MergeStrategy::Exact);

    let bottom_left_block = Block::bordered()
        .title("Bottom Left Block")
        .merge_borders(MergeStrategy::Exact);


    let kingdom_stats = get_kingdom_stats_as_list(game_state)
        .block(
            Block::bordered()
                .title("Kingdom Stats")
                .merge_borders(MergeStrategy::Exact)
        );



    frame.render_widget(right_block, terminal[1]);

    frame.render_widget(kingdom_stats, left[0]);

    frame.render_widget(bottom_left_block, left[1]);

}


fn get_kingdom_stats_as_list(game_state: &GameState) -> List<'static> {
    let kingdom = &game_state.kingdom;

    List::new(vec![
        ListItem::new(""),

        ListItem::new(format!(
            "Kingdom '{}', Ruled by '{}'",
            kingdom.name,
            kingdom.king
        )),

        ListItem::new(format!(
            "Kingdom level: {}",
            kingdom.level
        )),

        ListItem::new(""),

        ListItem::new(format!(
            "Resources:\n  Wood: {}\n  Stone: {}\n  Gold: {}\n  Food: {}",
            kingdom.resources.wood,
            kingdom.resources.stone,
            kingdom.resources.gold,
            kingdom.resources.food
        )),

        ListItem::new(""),

        ListItem::new(format!(
            "Army:\n  Knights: {}\n  Archers: {}",
            kingdom.army.knights,
            kingdom.army.archers
        )),
    ])
}