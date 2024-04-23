use std::fmt;
pub struct Task {
    pub id: u32,
    pub description: String,
    pub status:Status,
}

pub enum Status {
    Pending,
    Completed,
}
impl fmt::Display for Status{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::Pending => write!(f, "Pending"),
            Status::Completed => write!(f, "Completed"),
        }
    }
}