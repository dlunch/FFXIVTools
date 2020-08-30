use alloc::{boxed::Box, vec, vec::Vec};

use async_trait::async_trait;
use futures::channel::oneshot;
use hashbrown::HashMap;
use spinning_top::Spinlock;

use sqpack::{Package, Result, SqPackFileReference};

use crate::BatchablePackage;

pub struct BatchedPackage<'a> {
    real: Box<dyn BatchablePackage + 'a>,
    waiters: Spinlock<HashMap<SqPackFileReference, Vec<oneshot::Sender<Vec<u8>>>>>,
}

impl<'a> BatchedPackage<'a> {
    pub fn new<R: BatchablePackage + 'a>(real: R) -> Self {
        Self {
            real: Box::new(real),
            waiters: Spinlock::new(HashMap::new()),
        }
    }

    pub async fn poll(&self) -> Result<()> {
        if self.waiters.lock().is_empty() {
            return Ok(());
        }

        let waiters = {
            let mut waiters = self.waiters.lock();
            let mut new_waiters = HashMap::new();
            core::mem::swap(&mut *waiters, &mut new_waiters);

            new_waiters
        };

        let references = waiters.keys().collect::<Vec<_>>();
        let mut result = self.real.read_files(references.as_slice()).await?;

        for (reference, mut senders) in waiters.into_iter() {
            let value = result.remove(&reference).unwrap();

            if senders.len() == 1 {
                let sender = senders.pop().unwrap();
                sender.send(value).unwrap();
            } else {
                for sender in senders {
                    sender.send(value.clone()).unwrap();
                }
            }
        }

        Ok(())
    }
}

#[async_trait]
impl Package for BatchedPackage<'_> {
    async fn read_file_by_reference(&self, reference: &SqPackFileReference) -> Result<Vec<u8>> {
        let (tx, rx) = oneshot::channel();

        {
            let mut waiters = self.waiters.lock();
            if waiters.contains_key(reference) {
                waiters.get_mut(reference).unwrap().push(tx);
            } else {
                waiters.insert(reference.clone(), vec![tx]);
            }
        }

        Ok(rx.await.unwrap())
    }
}
