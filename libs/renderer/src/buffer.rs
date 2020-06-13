use alloc::{boxed::Box, sync::Arc};
use core::{ops::Drop, task::Poll};

pub struct Buffer {
    device: Arc<wgpu::Device>,
    pub(crate) buffer: Arc<wgpu::Buffer>,
    pub(crate) offset: usize,
    pub(crate) size: usize,
    free: Box<dyn Fn() + Sync + Send + 'static>,
}

impl Buffer {
    pub(crate) fn new<F>(device: Arc<wgpu::Device>, buffer: Arc<wgpu::Buffer>, offset: usize, size: usize, free: F) -> Self
    where
        F: Fn() + Sync + Send + 'static,
    {
        Self {
            device,
            buffer,
            offset,
            size,
            free: Box::new(free),
        }
    }

    pub async fn write(&self, data: &[u8]) -> Result<(), wgpu::BufferAsyncErr> {
        // TODO move poll to event loop
        let mut future = self.buffer.map_write(self.offset as u64, self.size as u64);

        let mut mapping;
        loop {
            if let Poll::Ready(x) = futures::poll!(&mut future) {
                mapping = x?;
                break;
            }
            self.device.poll(wgpu::Maintain::Wait);
        }

        mapping.as_slice().copy_from_slice(data);

        Ok(())
    }

    pub(crate) fn binding_resource(&self) -> wgpu::BindingResource {
        wgpu::BindingResource::Buffer {
            buffer: &self.buffer,
            range: self.offset as u64..self.offset as u64 + self.size as u64,
        }
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        (self.free)()
    }
}
