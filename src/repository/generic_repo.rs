pub trait GenericRepository<T> {
    async fn get(&self, id: &str) -> Result<T, Box<dyn std::error::Error>>;
    async fn create(&self, entity: T) -> Result<T, Box<dyn std::error::Error>>;
    async fn update(&self, entity: T) -> Result<T, Box<dyn std::error::Error>>;
    async fn delete(&self, id: &str) -> Result<(), Box<dyn std::error::Error>>;
    async fn list(&self) -> Result<Vec<T>, Box<dyn std::error::Error>>;
}
