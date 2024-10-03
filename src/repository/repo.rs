use crate::model::entity::Entity;
use crate::repository::generic_repo::GenericRepository;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::marker::PhantomData;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;
// use uuid::Uuid;

pub struct Repository<T: Entity + DeserializeOwned + Serialize> {
    pub db: Surreal<Client>,
    table_name: String,
    _marker: PhantomData<T>,
}

impl<T: Entity + DeserializeOwned + Serialize> Repository<T> {
    pub fn new(db: Surreal<Client>, table_name: &str) -> Self {
        Repository::<T> {
            db,
            table_name: table_name.to_string(),
            _marker: PhantomData,
        }
    }
}

impl<T: Entity + DeserializeOwned + Serialize> GenericRepository<T> for Repository<T> {
    async fn get(&self, id: &str) -> Result<T, Box<dyn std::error::Error>> {
        let entity: Option<T> = self.db.select((self.table_name.as_str(), id)).await?;

        match entity {
            Some(entity) => Ok(entity),
            None => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Entity not found",
            ))),
        }
    }

    async fn create(&self, entity: T) -> Result<T, Box<dyn std::error::Error>> {
        let json_data = serde_json::to_string(&entity)?;
        let res: Option<T> = self
            .db
            .create(self.table_name.as_str())
            .content(json_data)
            .await?;

        match res {
            Some(entity) => Ok(entity),
            None => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Entity not found",
            ))),
        }
    }

    // async fn create(&self, entity: T) -> Result<T, Box<dyn std::error::Error>> {
    //     let uuid = uuid::Uuid::new_v4().to_string();

    //     let query = format!(
    //         "CREATE {} SET id = '{}', data = '{}'",
    //         self.table_name,
    //         uuid,
    //         serde_json::to_string(&entity)?
    //     );
    //     self.db.query(&query).await?;

    //     Ok(entity)
    // }

    async fn update(&self, entity: T) -> Result<T, Box<dyn std::error::Error>> {
        let json_data = serde_json::to_string(&entity)?;
        let res: Option<T> = self
            .db
            .update((self.table_name.as_str(), entity.id()))
            .merge(json_data)
            .await?;

        match res {
            Some(entity) => Ok(entity),
            None => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Entity not found",
            ))),
        }
    }

    // async fn update(&self, entity: T) -> Result<T, Box<dyn std::error::Error>> {
    //     let query = format!(
    //         "UPDATE {} MERGE '{}'",
    //         self.table_name,
    //         serde_json::to_string(&entity)?
    //     );
    //     self.db.query(&query).await?;

    //     Ok(entity)
    // }

    async fn delete(&self, id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let res: Option<T> = self.db.delete((self.table_name.as_str(), id)).await?;

        match res {
            Some(_) => Ok(()),
            None => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Entity not found",
            ))),
        }
    }

    async fn list(&self) -> Result<Vec<T>, Box<dyn std::error::Error>> {
        let entities: Vec<T> = self.db.select(self.table_name.as_str()).await?;

        Ok(entities)
    }
}
