#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum ImportanceFilter {
    #[default]
    Any,
    High,
    Low,
    Neither,
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum UrgencyFilter {
    #[default]
    Any,
    High,
    Low,
    Neither,
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum PinnedFilter {
    #[default]
    Any,
    Pinned,
    Unpinned,
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum StateFilter {
    #[default]
    Any,
    Todo,
    InProgress,
    Blocked,
    Completed,
    Archived,
    None,
}
