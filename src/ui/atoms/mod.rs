//! Atomic UI primitives
//!
//! Following Atomic Design methodology, atoms are the fundamental building blocks.
//! Each atom is a single-purpose, self-contained UI primitive.
//!
//! # Design Principles
//!
//! 1. **Single Purpose**: Each atom does one thing well
//! 2. **No Dependencies**: Atoms don't depend on other atoms
//! 3. **Pure Rendering**: No mutable state, pure functions
//! 4. **Composable**: Can be combined into molecules
//! 5. **Testable**: 100% test coverage on all public APIs
//!
//! # Atoms
//!
//! - [`text`]: Styled text primitive
//! - [`block`]: Bordered container primitive
//! - [`icon`]: Nerd Font icon primitive
//!
//! # Examples
//!
//! ```
//! use toad::ui::atoms::text::Text;
//! use toad::ui::atoms::block::Block;
//! use toad::ui::atoms::icon::Icon;
//! use toad::ui::nerd_fonts::UiIcon;
//!
//! let text = Text::new("Hello").bold();
//! let block = Block::themed("Panel");
//! let icon = Icon::ui(UiIcon::Success);
//! ```

pub mod block;
pub mod icon;
pub mod text;

pub use block::Block;
pub use icon::Icon;
pub use text::Text;
