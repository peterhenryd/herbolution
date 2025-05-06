use bytemuck::{cast_slice, NoUninit, Pod};
use std::marker::PhantomData;
use std::ops::Deref;
use std::path::Path;
use image::{DynamicImage, GenericImageView};
use tracing::warn;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingResource, BindingType, Buffer, BufferAddress, BufferBindingType, BufferDescriptor, BufferUsages, Extent3d, ShaderStages, TexelCopyBufferLayout, TextureDescriptor, TextureDimension, TextureFormat, TextureSampleType, TextureUsages, TextureView, TextureViewDescriptor, TextureViewDimension};
use math::size::Size2;
use math::vector::{Vec2, Vec3, Vec4};
use crate::gpu::Gpu;

/// Represents a type that can provide a data payload suitable for GPU buffers.
///
/// This trait is used to abstract over types that might need some transformation
/// before being written to a GPU buffer (like adding padding or converting to a specific format).
pub trait Payload {
    /// The specific type of the data payload that will be written to the GPU buffer.
    /// This type must implement `NoUninit` to ensure it can re-interpreted as a byte slice.
    type Output: NoUninit;

    /// Returns the data payload representation of `self`.
    fn payload(&self) -> Self::Output;
}

/// A marker trait for types that are already in a suitable format for GPU buffers.
///
/// Types implementing `AutoPayload` must also implement `Copy` and `NoUninit`.
/// This trait allows these types to automatically implement the `Payload` trait.
pub trait AutoPayload: Copy + NoUninit {}

/// Automatic implementation of `Payload` for any type that implements `AutoPayload`.
///
/// For types marked with `AutoPayload`, the `payload` method simply returns a copy of the value itself.
impl<T: AutoPayload> Payload for T {
    type Output = T;

    fn payload(&self) -> Self::Output {
        *self
    }
}

// Blanket implementations of `AutoPayload` for common vector types.
impl<T: Pod> AutoPayload for Vec2<T> {}
impl<T: Pod> AutoPayload for Vec3<T> {}
impl<T: Pod> AutoPayload for Vec4<T> {}

/// A builder pattern structure for creating `BindGroup` and `BindGroupLayout` pairs.
///
/// This builder simplifies the process of defining the resources (buffers, textures, etc.)
/// that will be accessible to shaders and creating the necessary WGPU objects.
pub struct BindGroupBuilder<'a> {
    items: Vec<BindGroupItem<'a>>,
}

/// Represents a single entry within a `BindGroup` and its corresponding layout entry.
///
/// This struct bundles the `BindGroupLayoutEntry` (defining the type and visibility of the binding)
/// and the `BindGroupEntry` (linking the actual resource) for one binding point.
pub struct BindGroupItem<'a> {
    pub layout_entry: BindGroupLayoutEntry,
    pub group_entry: BindGroupEntry<'a>,
}

pub struct UniqueBindGroup {
    pub layout: BindGroupLayout,
    pub group: BindGroup,
}

/// A trait implemented by types that can be added to a `BindGroupBuilder`.
///
/// This allows different GPU resource wrappers (like `UniformBuffer` and `Texture`)
/// to define how they should be represented in a bind group.
pub trait AppendToBindGroup {
    /// Adds the necessary `BindGroupItem`(s) for this resource to the builder's list.
    ///
    /// # Arguments
    ///
    /// * `items` - The list of bind group items being collected by the `BindGroupBuilder`.
    /// * `visibility` - The shader stages (`ShaderStages`) where this resource should be accessible.
    fn append_to_bind_group<'a>(&'a self, items: &mut Vec<BindGroupItem<'a>>, visibility: ShaderStages);
}

impl<'a> BindGroupBuilder<'a> {
    /// Creates an empty `BindGroupBuilder`.
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
        }
    }

    /// Adds a pre-configured `BindGroupItem` to the builder.
    ///
    /// The binding index (`binding`) within the item's entries should typically be set
    /// based on the current number of items already added.
    pub fn add_item(&mut self, entry: BindGroupItem<'a>) {
        self.items.push(entry);
    }

    /// Adds a pre-configured `BindGroupItem` to the builder (returning `self`, allowing for chaining).
    pub fn with_item(mut self, entry: BindGroupItem<'a>) -> BindGroupBuilder<'a> {
        self.items.push(entry);
        self
    }

    /// Adds a resource that implements `AppendToBindGroup` to the builder.
    ///
    /// This method calls the resource's `append_to_bind_group` implementation, automatically
    /// determining the binding index based on the current number of items.
    ///
    /// # Arguments
    ///
    /// * `provider` - A reference to the resource to add (e.g., `&UniformBuffer<T>`, `&Texture`).
    /// * `visibility` - The shader stages where this resource should be accessible.
    pub fn append(mut self, provider: &'a impl AppendToBindGroup, visibility: ShaderStages) -> BindGroupBuilder<'a> {
        provider.append_to_bind_group(&mut self.items, visibility);
        self
    }

    /// Consumes the builder and creates the `BindGroup` and `BindGroupLayout` objects.
    ///
    /// It uses the collected `BindGroupItem`s to define the layout and bind the resources.
    ///
    /// # Arguments
    ///
    /// * `gpu` - A reference to the main `Gpu` context containing the WGPU device.
    ///
    /// # Returns
    ///
    /// A tuple containing the created `BindGroup` and `BindGroupLayout`.
    pub fn finish(self, gpu: &Gpu) -> UniqueBindGroup {
        let (layout_entries, group_entries): (Vec<_>, Vec<_>) = self.items
            .into_iter()
            .map(|entry| (entry.layout_entry, entry.group_entry))
            .unzip();

        let layout = gpu.device
            .create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: None,
                entries: &layout_entries,
            });
        let group = gpu.device
            .create_bind_group(&BindGroupDescriptor {
                label: None,
                layout: &layout,
                entries: &group_entries,
            });

        UniqueBindGroup { layout, group }
    }
}

/// A typed wrapper around a WGPU `Buffer` used for uniform data.
pub struct UniformBuffer<T> {
    buffer: Buffer,
    _marker: PhantomData<T>,
}

impl<T> UniformBuffer<T>
where T: Payload {
    /// Creates and initializes a new `UniformBuffer` with the given value.
    ///
    /// The buffer is created with `UNIFORM` and `COPY_DST` usages.
    ///
    /// # Arguments
    ///
    /// * `gpu` - A reference to the main `Gpu` context.
    /// * `value` - The initial value to store in the buffer. It will be converted using `T::payload`.
    pub fn create(gpu: &Gpu, value: &T) -> Self {
        Self {
            buffer: gpu.device
                .create_buffer_init(&BufferInitDescriptor {
                    label: None,
                    contents: cast_slice(&[value.payload()]),
                    usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
                }),
            _marker: PhantomData,
        }
    }

    /// Updates the contents of the uniform buffer with a new value.
    ///
    /// This performs a GPU queue operation to write the new data.
    ///
    /// # Arguments
    ///
    /// * `gpu` - A reference to the main `Gpu` context.
    /// * `value` - The new value to write to the buffer. It will be converted using `T::payload`.
    pub fn write(&self, gpu: &Gpu, value: &T) {
        gpu.queue.write_buffer(&self.buffer, 0, cast_slice(&[value.payload()]));
    }
}


impl<T> AppendToBindGroup for UniformBuffer<T> {
    fn append_to_bind_group<'a>(&'a self, entries: &mut Vec<BindGroupItem<'a>>, visibility: ShaderStages) {
        let binding = entries.len() as u32;
        entries.push(BindGroupItem {
            layout_entry: BindGroupLayoutEntry {
                binding,
                visibility,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            group_entry: BindGroupEntry {
                binding,
                resource: self.buffer.as_entire_binding(),
            },
        });
    }
}

// Type alias for clarity, representing the length field in the storage buffer.
type Len = u64;

/// A typed wrapper around a WGPU `Buffer` used for storage data, functioning as a resizable vector
/// with a pre-allocated static capacity.
///
/// Storage buffers are suitable for larger amounts of structured data that might be
/// read from or written to by shaders (compute shaders).
///
/// The buffer layout reserves the first `size_of::<Len>()` bytes to store the
/// current number of elements (`len`), followed by the actual data elements.
pub struct StorageBuffer<T>
where T: Payload {
    buffer: Buffer,
    // The number of elements currently written *to the GPU buffer*.
    len: Len,
    // A staging vector on the CPU to batch elements before writing to the GPU.
    push: Vec<T::Output>,
    clear: bool,
    _marker: PhantomData<T>,
}

impl<T> StorageBuffer<T>
where T: Payload {
    /// Creates a new `StorageBuffer` with a specified element capacity.
    ///
    /// The buffer is created with `STORAGE` and `COPY_DST` usages.
    /// The total buffer size accounts for the length field (`Len`) and the capacity for `T::Output` elements.
    ///
    /// # Arguments
    ///
    /// * `gpu` - A reference to the main `Gpu` context.
    /// * `capacity` - The maximum number of elements (`T::Output`) the buffer can hold.
    pub fn create(gpu: &Gpu, capacity: u64) -> Self {
        Self {
            buffer: gpu.device
                .create_buffer(&BufferDescriptor {
                    label: None,
                    size: size_of::<Len>() as BufferAddress + size_of::<T::Output>() as BufferAddress * capacity,
                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                    mapped_at_creation: false,
                }),
            len: 0,
            push: vec![],
            clear: false,
            _marker: PhantomData,
        }
    }

    /// Adds an element to the CPU-side staging vector.
    ///
    /// The element is converted using `T::payload`. It will only be written to the
    /// GPU buffer when `submit` is called. If the buffer is full (considering both
    /// elements already on the GPU and elements in the staging vector), a warning
    /// is logged and the element is discarded.
    ///
    /// # Arguments
    ///
    /// * `value` - The element to add.
    pub fn push(&mut self, value: &T){
        // Check if adding the element would exceed capacity
        if self.push.len() as u64 + self.len >= self.capacity() {
            warn!("Attempting to push to a full storage buffer");
            return;
        }

        self.push.push(value.payload());
    }

    /// Marks the buffer for clearing on the next `submit` call.
    ///
    /// This will ensure that only newly-pushed elements will exist on the buffer after the next GPU
    /// submission.
    pub fn clear(&mut self) {
        self.clear = true;
    }

    /// Writes the staged elements (from `push`) and the updated length to the GPU buffer.
    ///
    /// If the staging vector is empty, this function does nothing. Otherwise, it performs
    /// two `write_buffer` calls: one for the updated length at the beginning of the buffer,
    /// and one for the staged data elements appended after the existing data.
    /// The staging vector is cleared after the write.
    ///
    /// # Arguments
    ///
    /// * `gpu` - A reference to the main `Gpu` context.
    pub fn submit(&mut self, gpu: &Gpu) {
        if self.push.is_empty() {
            return;
        }

        if self.clear {
            self.len = 0;
            self.clear = false;
        }

        // Calculate the new total length on the GPU
        let len = self.len + self.push.len() as BufferAddress;
        // Write the new length to the start of the buffer
        gpu.queue.write_buffer(&self.buffer, 0, cast_slice(&[len]));

        // Calculate the offset where the new data should be written
        let offset = size_of::<Len>() as BufferAddress + self.len * size_of::<T::Output>() as BufferAddress;
        // Write the staged data from the `push` vector
        gpu.queue.write_buffer(&self.buffer, offset, cast_slice(&self.push));
        self.push.clear();

        self.len = len;
    }

    /// Returns the maximum number of elements (`T::Output`) the buffer can hold.
    pub fn capacity(&self) -> BufferAddress {
        (self.buffer.size() - size_of::<Len>() as BufferAddress) / size_of::<T::Output>() as BufferAddress
    }

    /// Returns the current number of elements (`T::Output`) written to the GPU buffer.
    pub fn len(&self) -> BufferAddress {
        self.len
    }
}

impl<T> AppendToBindGroup for StorageBuffer<T>
where T: Payload {
    fn append_to_bind_group<'a>(&'a self, entries: &mut Vec<BindGroupItem<'a>>, visibility: ShaderStages) {
        let binding = entries.len() as u32;
        entries.push(BindGroupItem {
            layout_entry: BindGroupLayoutEntry {
                binding,
                visibility,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Storage {
                        read_only: true,
                    },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            group_entry: BindGroupEntry {
                binding,
                resource: self.buffer.as_entire_binding(),
            },
        });
    }
}

/// A wrapper around a WGPU `Texture` and its associated `TextureView`.
pub struct Texture {
    inner: wgpu::Texture,
    pub(crate) view: TextureView,
}

impl Texture {
    /// Creates a `Texture` by loading an image from the specified file path.
    ///
    /// The image is decoded and converted to RGBA8 format before being uploaded to the GPU.
    ///
    /// # Arguments
    ///
    /// * `gpu` - A reference to the main `Gpu` context.
    /// * `path` - A path to the image file.
    ///
    /// # Returns
    ///
    /// An `image::ImageResult<Self>` which is `Ok(Texture)` on success or an `image::ImageError` on failure.
    pub fn open(gpu: &Gpu, path: impl AsRef<Path>) -> image::ImageResult<Self> {
        Ok(Self::from_image(gpu, image::open(path)?))
    }

    /// Creates a `Texture` from RGBA byte data.
    ///
    /// Assumes the bytes represent an image in a normalized RGBA8 format (`Rgba8UnormSrgb`).
    ///
    /// # Arguments
    ///
    /// * `gpu` - A reference to the main `Gpu` context.
    /// * `size` - The width and height of the texture.
    /// * `bytes` - A slice containing the raw RGBA pixel data.
    pub fn from_rgba_bytes(gpu: &Gpu, size: Size2<u32>, bytes: &[u8]) -> Self {
        // Create an empty texture with the correct format and size
        let texture = Self::empty(gpu, TextureFormat::Rgba8UnormSrgb, size);

        // Write the byte data to the texture
        gpu.queue.write_texture(
            texture.inner.as_image_copy(),
            bytes,
            TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(size.width * 4),
                rows_per_image: Some(size.height),
            },
            Extent3d { width: size.width, height: size.height, depth_or_array_layers: 1 },
        );

        texture
    }

    /// Creates a `Texture` from an `image::DynamicImage`.
    ///
    /// The image is converted to RGBA8 format before being uploaded.
    ///
    /// # Arguments
    ///
    /// * `gpu` - A reference to the main `Gpu` context.
    /// * `image` - The `DynamicImage` to upload.
    pub fn from_image(gpu: &Gpu, image: DynamicImage) -> Self {
        Self::from_rgba_bytes(
            gpu,
            Size2::from(image.dimensions()),
            &image.to_rgba8(),
        )
    }

    /// Creates an empty `Texture` with the specified format and size.
    ///
    /// The texture is created with `RENDER_ATTACHMENT`, `TEXTURE_BINDING`, and `COPY_DST` usages,
    /// making it suitable as a render target, for sampling in shaders, and as a destination for copy operations.
    ///
    /// # Arguments
    ///
    /// * `gpu` - A reference to the main `Gpu` context.
    /// * `format` - The `TextureFormat` for the texture pixels.
    /// * `size` - The width and height of the texture.
    pub fn empty(gpu: &Gpu, format: TextureFormat, size: Size2<u32>) -> Self {
        let size = Extent3d { width: size.width, height: size.height, depth_or_array_layers: 1 };
        let inner = gpu.device
            .create_texture(&TextureDescriptor {
                label: None,
                size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: TextureDimension::D2,
                format,
                usage: TextureUsages::RENDER_ATTACHMENT
                    | TextureUsages::TEXTURE_BINDING
                    | TextureUsages::COPY_DST,
                view_formats: &[],
            });
        let view = inner.create_view(&TextureViewDescriptor::default());

        Self { inner, view }
    }
}

impl AppendToBindGroup for Texture {
    fn append_to_bind_group<'a>(&'a self, items: &mut Vec<BindGroupItem<'a>>, visibility: ShaderStages) {
        let binding = items.len() as u32;
        items.push(BindGroupItem {
            layout_entry: BindGroupLayoutEntry {
                binding,
                visibility,
                ty: BindingType::Texture {
                    sample_type: TextureSampleType::Float {
                        filterable: true,
                    },
                    view_dimension: TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            group_entry: BindGroupEntry {
                binding,
                resource: BindingResource::TextureView(&self.view)
            },
        });
    }
}

/// A specialized wrapper for a `Texture` specifically used for depth buffering.
///
/// This is a new-type providing convenience methods for creating
/// resizable depth textures with the `Depth32Float` format.
pub struct DepthTexture(Texture);

impl DepthTexture {
    /// Creates a new `DepthTexture` with the specified size.
    ///
    /// The underlying texture uses the `TextureFormat::Depth32Float` format.
    ///
    /// # Arguments
    ///
    /// * `gpu` - A reference to the main `Gpu` context.
    /// * `size` - The width and height of the depth texture.
    pub fn create(gpu: &Gpu, size: Size2<u32>) -> Self {
        Self(Texture::empty(gpu, TextureFormat::Depth32Float, size))
    }

    /// Resizes the depth texture by creating a new underlying `Texture`.
    ///
    /// This discards the old texture and replaces it with a new one of the specified size.
    ///
    /// # Arguments
    ///
    /// * `gpu` - A reference to the main `Gpu` context.
    /// * `size` - The new width and height for the depth texture.
    pub fn set_size(&mut self, gpu: &Gpu, size: Size2<u32>) {
        self.0 = Texture::empty(gpu, TextureFormat::Depth32Float, size);
    }
}

impl Deref for DepthTexture {
    type Target = Texture;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}