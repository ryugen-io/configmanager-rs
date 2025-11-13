use crate::{
    state::{AppState, Pane, VimMode},
    theme::Theme,
};
use ratzilla::ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

pub fn render(f: &mut Frame, state: &AppState, area: Rect) {
    // Split status line into left and right sections
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(area);

    render_left(f, state, chunks[0]);
    render_right(f, chunks[1]);
}

fn render_left(f: &mut Frame, state: &AppState, area: Rect) {
    let mode_text = match state.vim_mode {
        VimMode::Normal => "NORMAL",
        VimMode::Insert => "INSERT",
    };

    let mode_color = match state.vim_mode {
        VimMode::Normal => Theme::NORMAL_MODE,
        VimMode::Insert => Theme::INSERT_MODE,
    };

    let mut spans = vec![Span::styled(
        format!(" {} ", mode_text),
        Style::default().fg(mode_color).add_modifier(Modifier::BOLD),
    )];

    // Only show file info when not in Menu
    if state.focus != Pane::Menu {
        spans.push(Span::raw(" | "));
        if let Some(filename) = &state.editor.current_file {
            spans.push(Span::styled(filename, Style::default().fg(Theme::TEXT)));
            if state.dirty {
                spans.push(Span::styled(
                    " [modified]",
                    Style::default().fg(Theme::MODIFIED),
                ));
            }
        } else {
            spans.push(Span::styled(
                "No file",
                Style::default().fg(Theme::SUBTEXT0),
            ));
        }
    }

    // Only show status message when not in Menu
    if state.focus != Pane::Menu
        && let Some(ref msg) = state.status_message
    {
        spans.push(Span::raw(" | "));
        spans.push(Span::styled(msg, Style::default().fg(Theme::SUCCESS)));
    }

    let help_text = match (state.focus, state.vim_mode) {
        (Pane::Menu, _) => " | j/k:navigate Enter:select",
        (Pane::FileList, _) => " | j/k:navigate Enter:load ESC:menu Ctrl-→:editor",
        (Pane::Editor, VimMode::Normal) => " | i:insert F2:save Ctrl-←:files",
        (Pane::Editor, VimMode::Insert) => " | ESC:normal F2:save",
    };
    spans.push(Span::styled(help_text, Style::default().fg(Theme::DIM)));

    let status_line = Paragraph::new(Line::from(spans))
        .style(Style::default().bg(Theme::MANTLE))
        .alignment(Alignment::Left);

    f.render_widget(status_line, area);
}

fn render_right(f: &mut Frame, area: Rect) {
    let tech_info = crate::version::tech_stack_info();

    let tech_line = Paragraph::new(Line::from(Span::styled(
        format!("{} ", tech_info),
        Style::default().fg(Theme::DIM),
    )))
    .style(Style::default().bg(Theme::MANTLE))
    .alignment(Alignment::Right);

    f.render_widget(tech_line, area);
}
