use std::collections::HashMap;

use async_trait::async_trait;
use tokio::sync::{oneshot, RwLock};

use crate::error::Result;
use crate::package::{BatchablePackage, Package};
use crate::reference::SqPackFileReference;

pub struct BatchedPackage<'a> {
    real: Box<dyn BatchablePackage + 'a>,
    waiters: RwLock<HashMap<SqPackFileReference, oneshot::Sender<Vec<u8>>>>,
}

impl<'a> BatchedPackage<'a> {
    pub fn new<R: BatchablePackage + 'a>(real: R) -> Self {
        Self {
            real: Box::new(real),
            waiters: RwLock::new(HashMap::new()),
        }
    }

    pub async fn poll(&self) -> Result<()> {
        let waiters = {
            let mut waiters = self.waiters.write().await;
            let mut new_waiters = HashMap::new();
            std::mem::swap(&mut *waiters, &mut new_waiters);

            new_waiters
        };

        let references = waiters.keys().into_iter().collect::<Vec<_>>();
        let mut result = self.real.read_many(references.as_slice()).await?;

        for (reference, sender) in waiters.into_iter() {
            let value = result.remove(&reference).unwrap();

            sender.send(value).unwrap();
        }

        Ok(())
    }
}

#[async_trait]
impl Package for BatchedPackage<'_> {
    async fn read_file_by_reference(&self, reference: &SqPackFileReference) -> Result<Vec<u8>> {
        let (tx, rx) = oneshot::channel();

        {
            let mut waiters = self.waiters.write().await;
            waiters.insert(reference.clone(), tx);
        }

        Ok(rx.await.unwrap())
    }
}
