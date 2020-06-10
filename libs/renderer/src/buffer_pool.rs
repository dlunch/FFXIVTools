use alloc::{sync::Arc, vec::Vec};

use crate::buffer::Buffer;

const BUFFER_SIZE: usize = 1048576;

pub struct BufferPoolItem {
    pub(crate) buffer: Arc<wgpu::Buffer>,
    offset: usize,
}

impl BufferPoolItem {
    pub fn new(device: &wgpu::Device) -> Self {
        let buffer = Arc::new(device.create_buffer(&wgpu::BufferDescriptor {
            size: BUFFER_SIZE as u64,
            usage: wgpu::BufferUsage::READ_ALL | wgpu::BufferUsage::WRITE_ALL,
            label: None,
        }));

        Self { buffer, offset: 0 }
    }

    pub fn alloc(&mut self, size: usize) -> Option<Buffer> {
        if self.offset + size > BUFFER_SIZE {
            None
        } else {
            let result = Buffer::new(self.buffer.clone(), self.offset, size);
            self.offset += size;

            Some(result)
        }
    }
}

pub struct BufferPool {
    items: Vec<BufferPoolItem>,
}

impl BufferPool {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn alloc(&mut self, device: &wgpu::Device, size: usize) -> Buffer {
        for item in &mut self.items {
            let result = item.alloc(size);
            if let Some(x) = result {
                return x;
            }
        }
        self.items.push(BufferPoolItem::new(device));

        let len = self.items.len();
        self.items[len - 1].alloc(size).unwrap()
    }
}
