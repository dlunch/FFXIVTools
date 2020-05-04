use core::task::Poll;

pub struct Buffer {
    buffer: wgpu::Buffer,
    size: usize,
}

impl Buffer {
    pub fn new(device: &wgpu::Device, size: usize) -> Self {
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: size as wgpu::BufferAddress,
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST | wgpu::BufferUsage::MAP_WRITE, // TODO temp
        });

        Self { buffer, size }
    }

    pub async fn write(&mut self, device: &wgpu::Device, data: &[u8]) -> Result<(), wgpu::BufferAsyncErr> {
        // TODO move poll to event loop
        let mut future = self.buffer.map_write(0, self.size as u64);

        let mut mapping: wgpu::BufferWriteMapping;
        loop {
            if let Poll::Ready(x) = futures::poll!(&mut future) {
                mapping = x?;
                break;
            }
            device.poll(wgpu::Maintain::Poll);
        }

        mapping.as_slice().copy_from_slice(data);

        Ok(())
    }

    pub(crate) fn binding_resource(&self) -> wgpu::BindingResource {
        wgpu::BindingResource::Buffer {
            buffer: &self.buffer,
            range: 0..self.size as u64,
        }
    }
}
