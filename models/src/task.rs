#[derive(Debug, PartialEq, Clone, Default)]
pub struct Task {
    pub id: usize,
    pub title: String,
    pub description: String,
    pub annotation: Option<String>,
    pub is_complete: bool,
}
