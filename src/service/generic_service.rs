pub trait GenericService<T> {
    fn handle_get(&self, id: &str) -> Result<T, Box<dyn std::error::Error>>;
    fn handle_create(&self, entity: T) -> Result<T, Box<dyn std::error::Error>>;
    fn handle_update(&self, entity: T) -> Result<T, Box<dyn std::error::Error>>;
    fn handle_delete(&self, id: &str) -> Result<(), Box<dyn std::error::Error>>;
    fn handle_list(&self) -> Result<Vec<T>, Box<dyn std::error::Error>>;
}
