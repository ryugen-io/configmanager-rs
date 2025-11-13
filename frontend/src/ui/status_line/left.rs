use crate::{
    state::{AppState, Pane, VimMode},
    theme::status_line::StatusLineTheme,
};
use ratzilla::ratatui::{
    Frame,
    layout::{Alignment, Rect},
    text::{Line, Span},
    widgets::Paragraph,
};

pub fn render(f: &mut Frame, state: &AppState, area: Rect) {
    let mode_text = StatusLineTheme::mode_text(state.vim_mode);
    let mode_style = StatusLineTheme::mode_style(state.vim_mode);

    let mut spans = vec![Span::styled(format!(" {} ", mode_text), mode_style)];

    // Only show file info when not in Menu
    if state.focus != Pane::Menu {
        spans.push(Span::raw(" | "));
        if let Some(filename) = &state.editor.current_file {
            spans.push(Span::styled(filename, StatusLineTheme::filename_style()));
            if state.dirty {
                spans.push(Span::styled(
                    " [modified]",
                    StatusLineTheme::modified_style(),
                ));
            }
        } else {
            spans.push(Span::styled("No file", StatusLineTheme::no_file_style()));
        }
    }

    // Only show status message when not in Menu
    if state.focus != Pane::Menu
        && let Some(ref msg) = state.status_message
    {
        spans.push(Span::raw(" | "));
        spans.push(Span::styled(msg, StatusLineTheme::status_message_style()));
    }

    let help_text = match (state.focus, state.vim_mode) {
        (Pane::Menu, _) => " | j/k:navigate Enter:select",
        (Pane::FileList, _) => " | j/k:navigate Enter:load ESC:menu Ctrl-→:editor",
        (Pane::Editor, VimMode::Normal) => " | i:insert F2:save Ctrl-←:files",
        (Pane::Editor, VimMode::Insert) => " | ESC:normal F2:save",
        (Pane::ContainerList, _) => " | j/k:navigate ESC/Ctrl-←:menu",
    };
    spans.push(Span::styled(help_text, StatusLineTheme::help_text_style()));

    let status_line = Paragraph::new(Line::from(spans))
        .style(StatusLineTheme::background())
        .alignment(Alignment::Left);

    f.render_widget(status_line, area);
}
