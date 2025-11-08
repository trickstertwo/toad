//! Navigation domain
//!
//! Contains navigation-related functionality including search, fuzzy finding,
//! recent files, and bookmarks.

pub mod advanced_search;
pub mod bookmarks;
pub mod fuzzy;
pub mod navigation;
pub mod recent_files;
pub mod search;

pub use advanced_search::{
    AdvancedSearchManager, AdvancedSearchMatch, FilterCondition, FilterHistory, FilterHistoryEntry,
    FilterOperator, MultiFieldFilter, SavedFilters,
};
pub use bookmarks::{Bookmark, BookmarkManager};
pub use fuzzy::{CaseMode, FuzzyFinder, FuzzyMatch, FuzzyMatcher, MatchStrategy};
pub use navigation::{NavigationAction, VimNavigation};
pub use recent_files::{RecentFile, RecentFiles};
pub use search::{SearchMatch, SearchState};
