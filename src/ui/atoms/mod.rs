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
//! - More atoms to come (block, button, icon, etc.)
//!
//! # Examples
//!
//! ```
//! use toad::ui::atoms::text::Text;
//!
//! let text = Text::new("Hello").bold();
//! ```

pub mod text;

pub use text::Text;
