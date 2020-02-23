use async_trait::async_trait;
use std::io;

#[async_trait]
pub trait Package {
    async fn read_file(&self, path: &str) -> io::Result<Vec<u8>>;
}
