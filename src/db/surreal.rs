use crate::cfg::config::Config;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::{Error, Surreal};

pub async fn connect(config: &Config) -> Result<Surreal<Client>, Error> {
    let db = Surreal::new::<Ws>("localhost:8000").await?;
    db.signin(Root {
        username: &config.surreal.user,
        password: &config.surreal.password,
    })
    .await?;

    db.use_ns(config.surreal.ns.as_str())
        .use_db(config.surreal.db_name.as_str())
        .await?;

    Ok(db)
}
