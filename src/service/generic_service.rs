pub trait GenericService<T> {
    async fn handle_get(&self, id: &str) -> Result<T, Box<dyn std::error::Error>>;
    async fn handle_create(&self, entity: T) -> Result<T, Box<dyn std::error::Error>>;
    async fn handle_update(&self, entity: T) -> Result<T, Box<dyn std::error::Error>>;
    async fn handle_delete(&self, id: &str) -> Result<(), Box<dyn std::error::Error>>;
    async fn handle_list(&self) -> Result<Vec<T>, Box<dyn std::error::Error>>;
}
