///! PSX Frogger game event handling

use crate::core::app::App;
use crate::core::app_state::AppScreen;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

impl App {
    /// Handle keys in PSX Frogger game
    pub(crate) fn handle_psx_frogger_key(&mut self, key: KeyEvent) -> crate::Result<()> {
        use crate::ui::widgets::FroggerGameState;

        match (key.code, key.modifiers) {
            // Escape returns to main screen
            (KeyCode::Esc, _) => {
                self.screen = AppScreen::Main;
                self.status_message = "Back to main screen".to_string();
            }
            // R restarts the game
            (KeyCode::Char('r'), _) | (KeyCode::Char('R'), _) => {
                self.psx_frogger.reset();
                self.status_message = "Game restarted! Go frogger!".to_string();
            }
            // Arrow keys move the player
            (KeyCode::Up, _) => {
                self.psx_frogger.move_player(0, -1);
            }
            (KeyCode::Down, _) => {
                self.psx_frogger.move_player(0, 1);
            }
            (KeyCode::Left, _) => {
                self.psx_frogger.move_player(-1, 0);
            }
            (KeyCode::Right, _) => {
                self.psx_frogger.move_player(1, 0);
            }
            // Space for level transition
            (KeyCode::Char(' '), _) => {
                if *self.psx_frogger.state() == FroggerGameState::LevelTransition {
                    self.psx_frogger.advance_to_next_level();
                    self.status_message = format!("Level {} - Let's hop!", self.psx_frogger.current_level());
                }
            }
            // Ctrl+C quits
            (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                self.should_quit = true;
            }
            _ => {}
        }

        // Update status message based on game state
        match self.psx_frogger.state() {
            FroggerGameState::Won => {
                self.status_message = format!(
                    "★ VICTORY! Final Score: {} | Press R to play again, ESC to quit",
                    self.psx_frogger.score()
                );
            }
            FroggerGameState::Lost => {
                self.status_message = format!(
                    "✗ Game Over! Score: {} | Press R to retry, ESC to quit",
                    self.psx_frogger.score()
                );
            }
            FroggerGameState::LevelTransition => {
                self.status_message = format!(
                    "▓ Level Complete! Press SPACE to continue to Level {}",
                    self.psx_frogger.current_level()
                );
            }
            FroggerGameState::Playing => {
                self.status_message = format!(
                    "Lives: {} | Score: {} | Time: {:.1}s | Theme: {}",
                    self.psx_frogger.lives(),
                    self.psx_frogger.score(),
                    self.psx_frogger.time_left(),
                    self.psx_frogger.theme().name()
                );
            }
        }

        Ok(())
    }
}
