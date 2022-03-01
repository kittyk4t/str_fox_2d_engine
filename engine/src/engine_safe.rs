use std::sync::Arc;
use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer, TypedBufferAccess};
use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage, SubpassContents};
use vulkano::descriptor_set::PersistentDescriptorSet;
use vulkano::device::physical::{PhysicalDevice, PhysicalDeviceType};
use vulkano::device::{Device, DeviceExtensions, Features};
use vulkano::format::Format;
use vulkano::image::ImageCreateFlags;
use vulkano::image::{
    view::ImageView, ImageAccess, ImageDimensions, ImageUsage, StorageImage, SwapchainImage,
};
use vulkano::instance::Instance;
use vulkano::pipeline::graphics::input_assembly::InputAssemblyState;
use vulkano::pipeline::graphics::vertex_input::BuffersDefinition;
use vulkano::pipeline::graphics::viewport::{Viewport, ViewportState};
use vulkano::pipeline::{GraphicsPipeline, Pipeline, PipelineBindPoint};
use vulkano::render_pass::{Framebuffer, RenderPass, Subpass};
use vulkano::sampler::{Filter, MipmapMode, Sampler, SamplerAddressMode};
use vulkano::swapchain::{self, AcquireError, Swapchain, SwapchainCreationError};
use vulkano::sync::{self, FlushError, GpuFuture};
use vulkano::Version;

use super::image::*;
use super::types::*;
use winit::event::{Event, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

pub const WIDTH: usize = 240;
pub const HEIGHT: usize = 240;

pub trait Game {
    type GameState;
    type Assets;
    type DrawState;
    fn new() -> (Self::GameState, Self::Assets);
    fn update(state: &mut Self::GameState, assets: &mut Self::Assets, keys: &Input);
    fn render(state: &mut Self::GameState, assets: &mut Self::Assets, fb: &mut Image);
}

pub struct Input {
    now_keys: Box<[bool]>,
    prev_keys: Box<[bool]>,
}
impl Input {
    fn new() -> Self {
        Self {
            now_keys: vec![false; 255].into_boxed_slice(),
            prev_keys: vec![false; 255].into_boxed_slice(),
        }
    }
    pub fn is_key_down(&self, kc: VirtualKeyCode) -> bool {
        self.now_keys[kc as usize]
    }
    pub fn is_key_up(&self, kc: VirtualKeyCode) -> bool {
        !self.now_keys[kc as usize]
    }
    pub fn is_key_pressed(&self, kc: VirtualKeyCode) -> bool {
        self.now_keys[kc as usize] && !self.prev_keys[kc as usize]
    }
    pub fn is_key_released(&self, kc: VirtualKeyCode) -> bool {
        !self.now_keys[kc as usize] && self.prev_keys[kc as usize]
    }
    fn next_frame(&mut self) {
        self.prev_keys.copy_from_slice(&self.now_keys);
    }
    fn handle_key_event(&mut self, ke: winit::event::KeyboardInput) {
        if let winit::event::KeyboardInput {
            virtual_keycode: Some(keycode),
            state,
            ..
        } = ke
        {
            match state {
                winit::event::ElementState::Pressed => {
                    self.now_keys[keycode as usize] = true;
                }
                winit::event::ElementState::Released => {
                    self.now_keys[keycode as usize] = false;
                }
            }
        }
    }
}

pub fn go<GameT: Game + 'static>() {
    let (mut state, mut assets) = GameT::new();
    let event_loop = EventLoop::new();
    let (mut vulkan_config, mut vulkan_state) = vulkan_init(&event_loop);
    let mut input = Input::new();
    event_loop.run(move |event, _, control_flow| {
        match event {
            // Nested match patterns are pretty useful---see if you can figure out what's going on in this match.
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(_),
                ..
            } => {
                vulkan_state.recreate_swapchain = true;
            }
            // NewEvents: Let's start processing events.
            Event::NewEvents(_) => {}
            // WindowEvent->KeyboardInput: Keyboard input!
            Event::WindowEvent {
                event:
                    WindowEvent::KeyboardInput {
                        input: in_event, ..
                    },
                ..
            } => {
                input.handle_key_event(in_event);
            }
            Event::MainEventsCleared => {
                GameT::update(&mut state, &mut assets, &input);
                GameT::render(
                    &mut state,
                    &mut assets,
                    &mut vulkan_state.framebuffer_data.fb2d,
                );
                render3d(&mut vulkan_config, &mut vulkan_state);
                input.next_frame();
            }
            _ => (),
        }
    });
}

fn best_present_mode(caps: vulkano::swapchain::Capabilities) -> vulkano::swapchain::PresentMode {
    [
        // vulkano::swapchain::PresentMode::Mailbox,
        // vulkano::swapchain::PresentMode::Immediate
    ]
    .into_iter()
    .find(|mode| caps.present_modes.supports(*mode))
    .unwrap_or(vulkano::swapchain::PresentMode::Fifo)
}

#[derive(Default, Debug, Clone)]
struct Vertex {
    position: [f32; 2],
    uv: [f32; 2],
}
vulkano::impl_vertex!(Vertex, position, uv);

struct VulkanConfig {
    surface: Arc<vulkano::swapchain::Surface<winit::window::Window>>,
    device: Arc<vulkano::device::Device>,
    queue: Arc<vulkano::device::Queue>,
    render_pass: Arc<vulkano::render_pass::RenderPass>,
    framebuffer_scheme: FramebufferScheme,
}

struct FramebufferScheme {
    // to draw a software framebuffer on a full screen triangle
    pipeline: Arc<vulkano::pipeline::GraphicsPipeline>,
    vertex_buffer: Arc<CpuAccessibleBuffer<[Vertex]>>,
    sampler: Arc<Sampler>,
}

impl FramebufferScheme {
    fn new(
        device: Arc<vulkano::device::Device>,
        render_pass: Arc<vulkano::render_pass::RenderPass>,
    ) -> Self {
        // We now create a buffer that will store the shape of our triangle.
        let vertex_buffer = CpuAccessibleBuffer::from_iter(
            device.clone(),
            BufferUsage::all(),
            false,
            [
                Vertex {
                    position: [-1.0, -1.0],
                    uv: [0.0, 0.0],
                },
                Vertex {
                    position: [3.0, -1.0],
                    uv: [2.0, 0.0],
                },
                Vertex {
                    position: [-1.0, 3.0],
                    uv: [0.0, 2.0],
                },
            ]
            .iter()
            .cloned(),
        )
        .unwrap();
        mod vs {
            vulkano_shaders::shader! {
                ty: "vertex",
                src: "
                #version 450

                layout(location = 0) in vec2 position;
                layout(location = 1) in vec2 uv;
                layout(location = 0) out vec2 out_uv;
                void main() {
                    gl_Position = vec4(position, 0.0, 1.0);
                    out_uv = uv;
                }
            "
            }
        }

        mod fs {
            vulkano_shaders::shader! {
                ty: "fragment",
                src: "
                #version 450

                layout(set = 0, binding = 0) uniform sampler2D tex;
                layout(location = 0) in vec2 uv;
                layout(location = 0) out vec4 f_color;

                void main() {
                    f_color = texture(tex, uv);
                }
            "
            }
        }

        let vs = vs::load(device.clone()).unwrap();
        let fs = fs::load(device.clone()).unwrap();

        let sampler = Sampler::new(
            device.clone(),
            Filter::Linear,
            Filter::Linear,
            MipmapMode::Nearest,
            SamplerAddressMode::Repeat,
            SamplerAddressMode::Repeat,
            SamplerAddressMode::Repeat,
            0.0,
            1.0,
            0.0,
            0.0,
        )
        .unwrap();

        let pipeline = GraphicsPipeline::start()
            .vertex_input_state(BuffersDefinition::new().vertex::<Vertex>())
            .vertex_shader(vs.entry_point("main").unwrap(), ())
            .input_assembly_state(InputAssemblyState::new())
            .viewport_state(ViewportState::viewport_dynamic_scissor_irrelevant())
            .fragment_shader(fs.entry_point("main").unwrap(), ())
            .render_pass(Subpass::from(render_pass, 0).unwrap())
            .build(device)
            .unwrap();

        Self {
            sampler,
            pipeline,
            vertex_buffer,
        }
    }
    fn create_framebuffer(
        &self,
        device: Arc<vulkano::device::Device>,
        qf: vulkano::device::physical::QueueFamily<'_>,
    ) -> FramebufferData {
        // We'll work on it locally, and copy it to a GPU buffer every frame.
        // Then on the GPU, we'll copy it into an Image.
        let buffer = CpuAccessibleBuffer::from_iter(
            device.clone(),
            BufferUsage::transfer_source(),
            false,
            (0..WIDTH * HEIGHT).map(|_| Color {
            r: 255_u8,
            g: 0_u8,
            b: 0_u8,
            a: 0_u8,
        }),
        )
        .unwrap();
        // Here's our (2D drawing) framebuffer.
        let fb2d = Image::new(Vec2i {
            x: WIDTH as i32,
            y: HEIGHT as i32,
        });
        // Let's set up the Image we'll copy into:
        let dimensions = ImageDimensions::Dim2d {
            width: WIDTH as u32,
            height: HEIGHT as u32,
            array_layers: 1,
        };
        let fb2d_image = StorageImage::with_usage(
            device,
            dimensions,
            Format::R8G8B8A8_UNORM,
            ImageUsage {
                // This part is key!
                transfer_destination: true,
                sampled: true,
                storage: true,
                transfer_source: false,
                color_attachment: false,
                depth_stencil_attachment: false,
                transient_attachment: false,
                input_attachment: false,
            },
            ImageCreateFlags::default(),
            std::iter::once(qf),
        )
        .unwrap();
        // Get a view on it to use as a texture:
        let fb2d_texture = ImageView::new(fb2d_image.clone()).unwrap();

        let layout = self
            .pipeline
            .layout()
            .descriptor_set_layouts()
            .get(0)
            .unwrap();
        let mut set_builder = PersistentDescriptorSet::start(layout.clone());

        set_builder
            .add_sampled_image(fb2d_texture, self.sampler.clone())
            .unwrap();

        let set = set_builder.build().unwrap();
        FramebufferData {
            fb2d,
            fb2d_image,
            set,
            buffer,
        }
    }
}

struct FramebufferData {
    // to draw specifically one framebuffer
    set: Arc<vulkano::descriptor_set::PersistentDescriptorSet>,
    fb2d_image: Arc<StorageImage>,
    fb2d: Image,
    buffer: Arc<CpuAccessibleBuffer<[Color]>>,
}

trait AutoCommandBufferBuilderFramebufferExt {
    fn copy_fb(
        &mut self,
        fb: &FramebufferData,
    ) -> Result<&mut Self, vulkano::command_buffer::CopyBufferImageError>;
    fn draw_fbs<'a>(
        &mut self,
        fb_scheme: &'a FramebufferScheme,
        fb_datas: impl Iterator<Item = &'a FramebufferData>,
    ) -> Result<&mut Self, vulkano::command_buffer::DrawError>;
}

impl<P, L> AutoCommandBufferBuilderFramebufferExt for AutoCommandBufferBuilder<P, L> {
    fn copy_fb(
        &mut self,
        fb: &FramebufferData,
    ) -> Result<&mut Self, vulkano::command_buffer::CopyBufferImageError> {
        {
            let writable_fb = &mut *fb.buffer.write().unwrap();
            writable_fb.copy_from_slice(fb.fb2d.as_slice());
        }
        self.copy_buffer_to_image(fb.buffer.clone(), fb.fb2d_image.clone())
    }
    fn draw_fbs<'a>(
        &mut self,
        fb_scheme: &'a FramebufferScheme,
        fb_datas: impl Iterator<Item = &'a FramebufferData>,
    ) -> Result<&mut Self, vulkano::command_buffer::DrawError> {
        let len = fb_scheme.vertex_buffer.len() as u32;
        self.bind_pipeline_graphics(fb_scheme.pipeline.clone())
            .bind_vertex_buffers(0, fb_scheme.vertex_buffer.clone());
        for fb in fb_datas {
            if let Err(err) = self
                .bind_descriptor_sets(
                    PipelineBindPoint::Graphics,
                    fb_scheme.pipeline.layout().clone(),
                    0,
                    fb.set.clone(),
                )
                .draw(len, 1, 0, 0)
            {
                return Err(err);
            }
        }
        Ok(self)
    }
}

struct VulkanState {
    swapchain: Arc<Swapchain<winit::window::Window>>,
    viewport: Viewport,
    framebuffers: Vec<Arc<vulkano::render_pass::Framebuffer>>,
    recreate_swapchain: bool,
    previous_frame_end: Option<Box<dyn vulkano::sync::GpuFuture>>,
    framebuffer_data: FramebufferData,
}

fn vulkan_init(event_loop: &EventLoop<()>) -> (VulkanConfig, VulkanState) {
    let required_extensions = vulkano::instance::InstanceExtensions {
        ext_debug_report: true,
        ..vulkano_win::required_extensions()
    };
    let instance = Instance::new(
        None,
        Version::V1_1,
        &required_extensions,
        vec!["VK_LAYER_KHRONOS_validation"],
    )
    .unwrap();

    use vulkano::instance::debug::{DebugCallback, MessageSeverity, MessageType};
    let _callback = DebugCallback::new(
        &instance,
        MessageSeverity::all(),
        MessageType::all(),
        |msg| {
            println!("Debug callback: {:?}", msg.description);
        },
    )
    .ok();
    use vulkano_win::VkSurfaceBuild;
    let surface = WindowBuilder::new()
        .build_vk_surface(event_loop, instance.clone())
        .unwrap();
    let device_extensions = DeviceExtensions {
        khr_swapchain: true,
        ..DeviceExtensions::none()
    };
    let (physical_device, queue_family) = PhysicalDevice::enumerate(&instance)
        .filter(|&p| p.supported_extensions().is_superset_of(&device_extensions))
        .filter_map(|p| {
            p.queue_families()
                .find(|&q| q.supports_graphics() && surface.is_supported(q).unwrap_or(false))
                .map(|q| (p, q))
        })
        .min_by_key(|(p, _)| match p.properties().device_type {
            PhysicalDeviceType::DiscreteGpu => 0,
            PhysicalDeviceType::IntegratedGpu => 1,
            PhysicalDeviceType::VirtualGpu => 2,
            PhysicalDeviceType::Cpu => 3,
            PhysicalDeviceType::Other => 4,
        })
        .unwrap();
    let (device, mut queues) = Device::new(
        physical_device,
        &Features::none(),
        &physical_device
            .required_extensions()
            .union(&device_extensions),
        [(queue_family, 0.5)].iter().cloned(),
    )
    .unwrap();
    let queue = queues.next().unwrap();
    let (swapchain, images) = {
        let caps = surface.capabilities(physical_device).unwrap();
        let composite_alpha = caps.supported_composite_alpha.iter().next().unwrap();
        let format = caps.supported_formats[0].0;
        let dimensions: [u32; 2] = surface.window().inner_size().into();

        Swapchain::start(device.clone(), surface.clone())
            .num_images(8)
            .format(format)
            .dimensions(dimensions)
            .usage(ImageUsage::color_attachment())
            .sharing_mode(&queue)
            .composite_alpha(composite_alpha)
            .present_mode(best_present_mode(caps))
            // .present_mode(mailbox or immediate or relaxed ??PresentMode::Fifo)
            // see if ^^^ fixes long waits on acquire, effectively vsynced framerate
            // mailbox or relaxed would be preferable, falling back to fifo if need be
            // then we can control frame timing and either do lockstep or interpolation
            .build()
            .unwrap()
    };
    let render_pass = vulkano::single_pass_renderpass!(
        device.clone(),
        attachments: {
            color: {
                // Pro move: We're going to cover the screen completely. Trust us!
                load: DontCare,
                store: Store,
                format: swapchain.format(),
                samples: 1,
            }
        },
        pass: {
            color: [color],
            depth_stencil: {}
        }
    )
    .unwrap();

    let framebuffer_scheme = FramebufferScheme::new(device.clone(), render_pass.clone());
    let framebuffer_data = framebuffer_scheme.create_framebuffer(device.clone(), queue_family);

    let mut viewport = Viewport {
        origin: [0.0, 0.0],
        dimensions: [0.0, 0.0],
        depth_range: 0.0..1.0,
    };

    let framebuffers = window_size_dependent_setup(&images, render_pass.clone(), &mut viewport);
    let recreate_swapchain = false;
    let previous_frame_end = Some(sync::now(device.clone()).boxed());

    (
        VulkanConfig {
            surface,
            device,
            render_pass,
            queue,
            framebuffer_scheme,
        },
        VulkanState {
            swapchain,
            viewport,
            framebuffers,
            recreate_swapchain,
            previous_frame_end,
            framebuffer_data,
        },
    )
}

fn render3d(vulkan_config: &mut VulkanConfig, vulkan_state: &mut VulkanState) {
    {
        if let Some(mut fut) = vulkan_state.previous_frame_end.take() {
            fut.cleanup_finished();
            // We need to synchronize here to send new data to the GPU.
            // We can't send the new framebuffer until the previous frame is done being drawn.
            // Dropping the future will block until it's done.
        }
    }

    if vulkan_state.recreate_swapchain {
        let dimensions: [u32; 2] = vulkan_config.surface.window().inner_size().into();
        let (new_swapchain, new_images) = match vulkan_state
            .swapchain
            .recreate()
            .dimensions(dimensions)
            .build()
        {
            Ok(r) => r,
            Err(SwapchainCreationError::UnsupportedDimensions) => return,
            Err(e) => panic!("Failed to recreate swapchain: {:?}", e),
        };

        vulkan_state.swapchain = new_swapchain;
        vulkan_state.framebuffers = window_size_dependent_setup(
            &new_images,
            vulkan_config.render_pass.clone(),
            &mut vulkan_state.viewport,
        );
        vulkan_state.recreate_swapchain = false;
    }

    let (image_num, suboptimal, acquire_future) =
        match swapchain::acquire_next_image(vulkan_state.swapchain.clone(), None) {
            Ok(r) => r,
            Err(AcquireError::OutOfDate) => {
                vulkan_state.recreate_swapchain = true;
                return;
            }
            Err(e) => panic!("Failed to acquire next image: {:?}", e),
        };
    if suboptimal {
        vulkan_state.recreate_swapchain = true;
    }

    let mut builder = AutoCommandBufferBuilder::primary(
        vulkan_config.device.clone(),
        vulkan_config.queue.family(),
        CommandBufferUsage::OneTimeSubmit,
    )
    .unwrap();

    builder
        // copy our FB
        .copy_fb(&vulkan_state.framebuffer_data)
        .unwrap()
        // And resume our regularly scheduled programming
        .begin_render_pass(
            vulkan_state.framebuffers[image_num].clone(),
            SubpassContents::Inline,
            std::iter::once(vulkano::format::ClearValue::None),
        )
        .unwrap()
        .set_viewport(0, [vulkan_state.viewport.clone()])
        .draw_fbs(
            &vulkan_config.framebuffer_scheme,
            std::iter::once(&vulkan_state.framebuffer_data),
        )
        .unwrap()
        .end_render_pass()
        .unwrap();

    let command_buffer = builder.build().unwrap();

    let future = acquire_future
        .then_execute(vulkan_config.queue.clone(), command_buffer)
        .unwrap()
        .then_swapchain_present(
            vulkan_config.queue.clone(),
            vulkan_state.swapchain.clone(),
            image_num,
        )
        .then_signal_fence_and_flush();

    match future {
        Ok(future) => {
            vulkan_state.previous_frame_end = Some(future.boxed());
        }
        Err(FlushError::OutOfDate) => {
            vulkan_state.recreate_swapchain = true;
            vulkan_state.previous_frame_end = Some(sync::now(vulkan_config.device.clone()).boxed());
        }
        Err(e) => {
            println!("Failed to flush future: {:?}", e);
            vulkan_state.previous_frame_end = Some(sync::now(vulkan_config.device.clone()).boxed());
        }
    }
}

fn window_size_dependent_setup(
    images: &[Arc<SwapchainImage<Window>>],
    render_pass: Arc<RenderPass>,
    viewport: &mut Viewport,
) -> Vec<Arc<Framebuffer>> {
    let dimensions = images[0].dimensions().width_height();
    viewport.dimensions = [dimensions[0] as f32, dimensions[1] as f32];

    images
        .iter()
        .map(|image| {
            let view = ImageView::new(image.clone()).unwrap();
            Framebuffer::start(render_pass.clone())
                .add(view)
                .unwrap()
                .build()
                .unwrap()
        })
        .collect::<Vec<_>>()
}
