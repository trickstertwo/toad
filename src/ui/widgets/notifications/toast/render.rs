//! Toast rendering implementations

use super::{Toast, ToastLevel, ToastManager};
use crate::ui::theme::ToadTheme;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

impl Toast {
    /// Render a single toast
    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let icon = self.level.icon();
        let color = self.level.border_color();

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(color))
            .style(Style::default().bg(ToadTheme::BLACK));

        let inner = block.inner(area);
        frame.render_widget(block, area);

        let text = Line::from(vec![
            Span::styled(
                format!("{} ", icon),
                Style::default().fg(color).add_modifier(Modifier::BOLD),
            ),
            Span::styled(&self.message, Style::default().fg(ToadTheme::FOREGROUND)),
        ]);

        let paragraph = Paragraph::new(text).alignment(Alignment::Left);
        frame.render_widget(paragraph, inner);
    }
}

impl ToastManager {
    /// Render all visible toasts
    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        // Auto-cleanup expired toasts
        self.cleanup();

        if self.toasts.is_empty() {
            return;
        }

        // Calculate layout for toasts (stack from top)
        let toast_height = 3; // Height of each toast
        let max_toasts = (area.height as usize) / toast_height;
        let visible_toasts = self.toasts.len().min(max_toasts);

        let constraints: Vec<Constraint> = (0..visible_toasts)
            .map(|_| Constraint::Length(toast_height as u16))
            .collect();

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(area);

        // Render each visible toast
        for (i, toast) in self.toasts.iter().take(visible_toasts).enumerate() {
            let toast_area = chunks[i];
            toast.render(frame, toast_area);
        }
    }
}
