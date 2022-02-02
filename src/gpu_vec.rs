use mem_align::MemAlign;
use wgpu::*;
use std::mem::size_of;

pub struct GPUVec<T: Copy> {
    buffer: wgpu::Buffer,
    // mem_align: MemAlign<T>,
    capacity: usize,
    phantom: std::marker::PhantomData<T>,
}

impl<T: Copy> GPUVec<T> {
    pub fn new(device: &wgpu::Device, capacity: usize, label: &str) -> Self {

        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some(label),
            size: (size_of::<T>() * capacity) as u64,
            usage: BufferUsages::STORAGE,
            mapped_at_creation: true,
        });

        Self { buffer, capacity, phantom: Default::default() }
    }

    pub fn new_uniforms(device: &wgpu::Device, label: &str) -> Self {

        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some(label),
            size: size_of::<T>() as _,
            usage: BufferUsages::UNIFORM,
            mapped_at_creation: true,
        });

        Self { buffer, capacity: 1, phantom: Default::default() }
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn buffer(&self) -> &wgpu::Buffer {
        &self.buffer
    }

    pub fn bind_group_entry(&self, binding: u32) -> wgpu::BindGroupEntry {
        wgpu::BindGroupEntry {
            binding: binding,
            resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                buffer: &self.buffer,
                offset: 0,
                size: None,
            }),
        }
    }

    pub fn unmap(&self) {
        self.buffer.unmap();
    }

    pub fn set(&mut self, index: usize, value: T) {
        let mut view = self.buffer.slice(..).get_mapped_range_mut();
        let slice = &mut *view;
        let slice2 =
            unsafe { std::slice::from_raw_parts_mut(slice.as_ptr() as *mut T, self.capacity) };
        slice2[index] = value;
    }
}

impl<T: Copy> std::ops::Index<usize> for GPUVec<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        let view = self.buffer.slice(..).get_mapped_range();
        let slice =
            unsafe { std::slice::from_raw_parts(view.as_ptr() as *const T, self.capacity) };
        &slice[index]
    }
}

impl<T: Copy> std::ops::IndexMut<usize> for GPUVec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let mut view = self.buffer.slice(..).get_mapped_range_mut();
        let slice =
            unsafe { std::slice::from_raw_parts_mut(view.as_mut_ptr() as *mut T, self.capacity) };
        &mut slice[index]
    }
}
