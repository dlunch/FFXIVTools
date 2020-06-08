use core::task::Poll;

pub struct UniformBuffer {
    buffer: wgpu::Buffer,
    size: usize,
}

impl UniformBuffer {
    pub fn new(device: &wgpu::Device, size: usize) -> Self {
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: size as wgpu::BufferAddress,
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::MAP_WRITE,
        });

        Self { buffer, size }
    }

    pub async fn write(&mut self, device: &wgpu::Device, data: &[u8]) -> Result<(), wgpu::BufferAsyncErr> {
        // TODO move poll to event loop
        let mut future = self.buffer.map_write(0, self.size as u64);

        let mut mapping;
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
