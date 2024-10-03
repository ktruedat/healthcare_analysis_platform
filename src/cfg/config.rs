use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub surreal: SurrealConfig,
}

#[derive(Debug, Deserialize)]
pub struct SurrealConfig {
    pub user: String,
    pub password: String,
    pub ns: String,
    pub db_name: String,
}
