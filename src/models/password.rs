/// Represents a password record in the database
#[derive(Debug)]
pub struct Password {
    pub id: String,
    pub password: String,
}