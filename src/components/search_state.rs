//! Shared search state
//!
//! Search state used by viewer components (log viewer, diff viewer, etc.).

/// Search state for viewer components
#[derive(Debug, Clone, Default)]
pub struct SearchState {
    /// Whether search is active
    pub active: bool,
    /// Current search query
    pub query: String,
    /// Line indices that match the query
    pub matches: Vec<usize>,
    /// Current match index
    pub current_match: usize,
}
