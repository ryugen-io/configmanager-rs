use crate::theme::status_line::StatusLineTheme;
use ratzilla::ratatui::{
    Frame,
    layout::{Alignment, Rect},
    text::{Line, Span},
    widgets::Paragraph,
};

pub fn render(f: &mut Frame, area: Rect) {
    let build_date = env!("BUILD_DATE");
    let ratzilla_version = env!("RATZILLA_VERSION");
    let ratatui_version = env!("RATATUI_VERSION");
    let axum_version = env!("AXUM_VERSION");

    let spans = vec![
        Span::styled("built: ", StatusLineTheme::label_style()),
        Span::styled(build_date, StatusLineTheme::value_style()),
        Span::styled(" | Ratzilla v", StatusLineTheme::label_style()),
        Span::styled(ratzilla_version, StatusLineTheme::value_style()),
        Span::styled(" | Ratatui v", StatusLineTheme::label_style()),
        Span::styled(ratatui_version, StatusLineTheme::value_style()),
        Span::styled(" | Axum v", StatusLineTheme::label_style()),
        Span::styled(format!("{} ", axum_version), StatusLineTheme::value_style()),
    ];

    let tech_line = Paragraph::new(Line::from(spans))
        .style(StatusLineTheme::background())
        .alignment(Alignment::Right);

    f.render_widget(tech_line, area);
}
