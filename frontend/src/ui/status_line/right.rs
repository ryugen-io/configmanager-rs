use crate::theme::Theme;
use ratzilla::ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::Style,
    text::{Line, Span},
    widgets::Paragraph,
};

pub fn render(f: &mut Frame, area: Rect) {
    let build_date = env!("BUILD_DATE");
    let ratzilla_version = env!("RATZILLA_VERSION");
    let ratatui_version = env!("RATATUI_VERSION");
    let axum_version = env!("AXUM_VERSION");

    let spans = vec![
        Span::styled("built: ", Style::default().fg(Theme::DIM)),
        Span::styled(build_date, Style::default().fg(Theme::YELLOW)),
        Span::styled(" | Ratzilla v", Style::default().fg(Theme::DIM)),
        Span::styled(ratzilla_version, Style::default().fg(Theme::YELLOW)),
        Span::styled(" | Ratatui v", Style::default().fg(Theme::DIM)),
        Span::styled(ratatui_version, Style::default().fg(Theme::YELLOW)),
        Span::styled(" | Axum v", Style::default().fg(Theme::DIM)),
        Span::styled(
            format!("{} ", axum_version),
            Style::default().fg(Theme::YELLOW),
        ),
    ];

    let tech_line = Paragraph::new(Line::from(spans))
        .style(Style::default().bg(Theme::MANTLE))
        .alignment(Alignment::Right);

    f.render_widget(tech_line, area);
}
