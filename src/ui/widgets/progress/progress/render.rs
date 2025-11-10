//! Progress widget rendering implementations

use super::MultiStageProgress;
use crate::ui::theme::ToadTheme;
use ratatui::{buffer::Buffer, layout::Rect, style::Style, widgets::Widget};

impl Widget for &MultiStageProgress {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width == 0 || area.height == 0 {
            return;
        }

        let stages_str = self.render_string();
        let style = Style::default().fg(ToadTheme::TOAD_GREEN);

        // Render stage indicators
        buf.set_string(area.x, area.y, stages_str, style);
    }
}
