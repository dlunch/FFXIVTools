use alloc::sync::Arc;
use core::task::Poll;

pub struct Buffer {
    buffer: Arc<wgpu::Buffer>,
    offset: usize,
    size: usize,
}

impl Buffer {
    pub(crate) fn new(buffer: Arc<wgpu::Buffer>, offset: usize, size: usize) -> Self {
        Self { buffer, offset, size }
    }

    pub async fn write(&self, device: &wgpu::Device, data: &[u8]) -> Result<(), wgpu::BufferAsyncErr> {
        // TODO move poll to event loop
        let mut future = self.buffer.map_write(0, self.size as u64);

        let mut mapping;
        loop {
            if let Poll::Ready(x) = futures::poll!(&mut future) {
                mapping = x?;
                break;
            }
            device.poll(wgpu::Maintain::Wait);
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
