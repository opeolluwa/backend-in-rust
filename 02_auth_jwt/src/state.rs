use sea_orm::DatabaseConnection;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}

impl AppState {
    pub fn from(db: &DatabaseConnection) -> Self {
        Self { db: db.clone() }
    }
}
