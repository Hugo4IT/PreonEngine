pub trait BufferLayout {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a>;
}

pub struct InstanceBuffer<T>
where
    T: Copy + Clone + bytemuck::Pod + bytemuck::Zeroable + BufferLayout,
{
    instances: Vec<T>,
    buffer: wgpu::Buffer,
    previous_size: Option<usize>,
}

impl<T> InstanceBuffer<T>
where
    T: Copy + Clone + bytemuck::Pod + bytemuck::Zeroable + BufferLayout,
{
    pub fn new<D: wgpu::util::DeviceExt>(device: &D) -> Self {
        let instances: Vec<T> = Vec::new();
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("InstanceBuffer"),
            contents: bytemuck::cast_slice(instances.as_slice()),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        Self {
            instances,
            buffer,
            previous_size: None,
        }
    }

    pub fn len(&self) -> usize {
        self.instances.len()
    }

    pub fn get(&self) -> wgpu::BufferSlice {
        self.buffer.slice(..)
    }

    pub fn begin(&mut self) {
        self.previous_size = Some(self.instances.len());
        self.instances.clear();
    }

    pub fn push(&mut self, item: T) {
        self.instances.push(item);
    }

    pub fn end<D: wgpu::util::DeviceExt>(&mut self, device: &D, queue: &wgpu::Queue) {
        if self.previous_size.expect(
            "InstanceBuffer<_>::begin(..); was not called before InstanceBuffer<_>::end(..);",
        ) != self.instances.len()
        {
            self.buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("New InstanceBuffer"),
                contents: bytemuck::cast_slice(self.instances.as_slice()),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            });
        } else {
            queue.write_buffer(
                &self.buffer,
                0,
                bytemuck::cast_slice(self.instances.as_slice()),
            );
        }
    }
}
