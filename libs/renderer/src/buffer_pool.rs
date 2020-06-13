use alloc::{sync::Arc, vec::Vec};

use spinning_top::Spinlock;

use crate::buffer::Buffer;

const BUFFER_SIZE: usize = 1048576;

struct BufferPoolItem {
    buffer: Arc<wgpu::Buffer>,
    offset: usize,
    size: usize,
    allocated: usize,
}

impl BufferPoolItem {
    pub fn new(device: &wgpu::Device) -> Self {
        let buffer = Arc::new(device.create_buffer(&wgpu::BufferDescriptor {
            size: BUFFER_SIZE as u64,
            usage: wgpu::BufferUsage::READ_ALL | wgpu::BufferUsage::WRITE_ALL,
            label: None,
        }));

        Self {
            buffer,
            offset: 0,
            size: BUFFER_SIZE,
            allocated: 0,
        }
    }

    pub fn alloc(&mut self, size: usize) -> Option<(Arc<wgpu::Buffer>, usize)> {
        if self.allocated > self.size {
            None
        } else {
            let offset = self.offset;
            self.offset += size;
            self.allocated += size;

            Some((self.buffer.clone(), offset))
        }
    }

    #[allow(unused_variables)]
    pub fn free(&mut self, offset: usize) {
        // TODO
    }
}

pub struct BufferPool {
    device: Arc<wgpu::Device>,
    items: Spinlock<Vec<Arc<Spinlock<BufferPoolItem>>>>,
}

impl BufferPool {
    pub fn new(device: Arc<wgpu::Device>) -> Self {
        Self {
            device,
            items: Spinlock::new(Vec::new()),
        }
    }

    pub fn alloc(&self, size: usize) -> Buffer {
        let mut items = self.items.lock();

        for item in &*items {
            let result = self.try_alloc(&item, size);
            if let Some(x) = result {
                return x;
            }
        }
        items.push(Arc::new(Spinlock::new(BufferPoolItem::new(&self.device))));
        self.try_alloc(items.last().unwrap(), size).unwrap()
    }

    fn try_alloc(&self, buffer_item: &Arc<Spinlock<BufferPoolItem>>, size: usize) -> Option<Buffer> {
        let (buffer, offset) = buffer_item.lock().alloc(size)?;

        let buffer_item = buffer_item.clone();
        Some(Buffer::new(self.device.clone(), buffer, offset, size, move || {
            buffer_item.lock().free(offset)
        }))
    }
}
