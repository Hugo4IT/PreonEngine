#[derive(Debug, Clone, Copy)]
pub enum PreonLayout {
    Rows,
    Columns,
    Container,
}

impl Default for PreonLayout {
    fn default() -> Self {
        Self::Rows
    }
}