use async_trait::async_trait;

use crate::error::Result;

#[async_trait]
pub trait Executor {
    type Output;
    async fn exec(&self) -> Result<Self::Output>;
}
