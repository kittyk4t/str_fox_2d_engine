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

use winit::event::{Event, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

use super::image::Image;
use super::types::*;

const WIDTH: usize = 320;
const HEIGHT: usize = 240;

struct Assets {
    img: Image,
    colors: [Color; 6],
}

struct State {
    w: usize,
    y: usize,
    color: usize,
}

fn main() {
    let assets = Assets {
        img: Image::from_file(std::path::Path::new("content/king.png")),
        colors: [
            Color::new(255, 0, 0, 255),
            Color::new(255, 255, 0, 255),
            Color::new(0, 255, 0, 255),
            Color::new(0, 255, 255, 255),
            Color::new(0, 0, 255, 255),
            Color::new(255, 0, 255, 255),
        ],
    };
    let state = State {
        w: WIDTH / 2,
        y: 32,
        color: 0,
    };

    go(state, assets, update, render2d, WIDTH, HEIGHT);
}

fn update(now_keys: &[bool], state: &mut State, assets: &Assets) {
    // We can actually handle events now that we know what they all are.
    if now_keys[VirtualKeyCode::Up as usize] {
        state.color = (state.color + 1) % assets.colors.len();
    }
    if now_keys[VirtualKeyCode::Down as usize] {
        // What is this if doing?
        state.color = if state.color == 0 {
            assets.colors.len() - 1
        } else {
            state.color - 1
        };
    }
    if now_keys[VirtualKeyCode::Left as usize] && state.w > 0 {
        state.w = if state.w < 4 { 0 } else { state.w - 4 };
    }
    if now_keys[VirtualKeyCode::Right as usize] && state.w < WIDTH - 1 {
        state.w = (state.w + 4).min(WIDTH - 1);
    }
    // Exercise for the reader: Tie y to mouse movement
    state.y = (state.y + 3) % HEIGHT;
}

fn render2d(assets: &Assets, state: &State, fb2d: &mut Image) {
    fb2d.clear(Color::new(128, 64, 64, 255));
    fb2d.bitblt(
        &assets.img,
        Rect {
            pos: Vec2i { x: 0, y: 16 },
            sz: Vec2i { x: 16, y: 16 },
        },
        Vec2i {
            x: fb2d.sz.x / 2,
            y: fb2d.sz.y / 2,
        },
    );
    // Then draw our new line:
    fb2d.hline(
        WIDTH / 2 - state.w / 2,
        WIDTH / 2 + state.w / 2,
        state.y,
        assets.colors[state.color],
    );
}

fn best_present_mode(caps: vulkano::swapchain::Capabilities) -> vulkano::swapchain::PresentMode {
    [
        // vulkano::swapchain::PresentMode::Mailbox,
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

pub struct VulkanConfig {
    surface: Arc<vulkano::swapchain::Surface<winit::window::Window>>,
    device: Arc<vulkano::device::Device>,
    set: Arc<vulkano::descriptor_set::PersistentDescriptorSet>,
    pipeline: Arc<vulkano::pipeline::GraphicsPipeline>,
    vertex_buffer: Arc<CpuAccessibleBuffer<[Vertex]>>,
    fb2d_image: Arc<StorageImage>,
    fb2d_buffer: Arc<CpuAccessibleBuffer<[Color]>>,
    pub fb2d: Image,
    queue: Arc<vulkano::device::Queue>,
    render_pass: Arc<vulkano::render_pass::RenderPass>,
}

pub struct VulkanState {
    swapchain: Arc<Swapchain<winit::window::Window>>,
    viewport: Viewport,
    framebuffers: Vec<Arc<vulkano::render_pass::Framebuffer>>,
    pub recreate_swapchain: bool,
    previous_frame_end: Option<Box<dyn vulkano::sync::GpuFuture>>,
}

pub fn vulkan_init(
    event_loop: &EventLoop<()>,
    width: usize,
    height: usize,
) -> (VulkanConfig, VulkanState) {
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
    // Here's our (2D drawing) framebuffer.
    let fb2d = Image::new(Vec2i {
        x: width as i32,
        y: height as i32,
    });
    // We'll work on it locally, and copy it to a GPU buffer every frame.
    // Then on the GPU, we'll copy it into an Image.
    let fb2d_buffer = CpuAccessibleBuffer::from_iter(
        device.clone(),
        BufferUsage::transfer_source(),
        false,
        (0..width * height).map(|_| Color {
            r: 255_u8,
            g: 0_u8,
            b: 0_u8,
            a: 0_u8,
        }),
    )
    .unwrap();
    // Let's set up the Image we'll copy into:
    let dimensions = ImageDimensions::Dim2d {
        width: width as u32,
        height: height as u32,
        array_layers: 1,
    };
    let fb2d_image = StorageImage::with_usage(
        device.clone(),
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
        std::iter::once(queue_family),
    )
    .unwrap();
    // Get a view on it to use as a texture:
    let fb2d_texture = ImageView::new(fb2d_image.clone()).unwrap();

    let fb2d_sampler = Sampler::new(
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
    let pipeline = GraphicsPipeline::start()
        .vertex_input_state(BuffersDefinition::new().vertex::<Vertex>())
        .vertex_shader(vs.entry_point("main").unwrap(), ())
        .input_assembly_state(InputAssemblyState::new())
        .viewport_state(ViewportState::viewport_dynamic_scissor_irrelevant())
        .fragment_shader(fs.entry_point("main").unwrap(), ())
        .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
        .build(device.clone())
        .unwrap();

    let layout = pipeline.layout().descriptor_set_layouts().get(0).unwrap();
    let mut set_builder = PersistentDescriptorSet::start(layout.clone());

    set_builder
        .add_sampled_image(fb2d_texture, fb2d_sampler)
        .unwrap();

    let set = set_builder.build().unwrap();

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
            set,
            pipeline,
            render_pass,
            vertex_buffer,
            fb2d_image,
            fb2d_buffer,
            fb2d,
            queue,
        },
        VulkanState {
            swapchain,
            viewport,
            framebuffers,
            recreate_swapchain,
            previous_frame_end,
        },
    )
}

pub fn render3d(vulkan_config: &mut VulkanConfig, vulkan_state: &mut VulkanState) {
    {
        if let Some(mut fut) = vulkan_state.previous_frame_end.take() {
            fut.cleanup_finished();
            // We need to synchronize here to send new data to the GPU.
            // We can't send the new framebuffer until the previous frame is done being drawn.
            // Dropping the future will block until it's done.
        }
    }

    // Now we can try to copy our buffer.
    {
        let writable_fb = &mut *vulkan_config.fb2d_buffer.write().unwrap();
        writable_fb.copy_from_slice(vulkan_config.fb2d.as_slice());
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
        // Now copy that framebuffer buffer into the framebuffer image
        .copy_buffer_to_image(
            vulkan_config.fb2d_buffer.clone(),
            vulkan_config.fb2d_image.clone(),
        )
        .unwrap()
        // And resume our regularly scheduled programming
        .begin_render_pass(
            vulkan_state.framebuffers[image_num].clone(),
            SubpassContents::Inline,
            std::iter::once(vulkano::format::ClearValue::None),
        )
        .unwrap()
        .set_viewport(0, [vulkan_state.viewport.clone()])
        .bind_pipeline_graphics(vulkan_config.pipeline.clone())
        .bind_descriptor_sets(
            PipelineBindPoint::Graphics,
            vulkan_config.pipeline.layout().clone(),
            0,
            vulkan_config.set.clone(),
        )
        .bind_vertex_buffers(0, vulkan_config.vertex_buffer.clone())
        .draw(vulkan_config.vertex_buffer.len() as u32, 1, 0, 0)
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

// area for improvement: all these 'static bounds are gnarly
fn go<StateT, AssetsT>(
    mut s: StateT,
    a: AssetsT,
    update: impl Fn(&[bool], &mut StateT, &AssetsT) + 'static,
    render: impl Fn(&AssetsT, &StateT, &mut Image) + 'static,
    width: usize,
    height: usize,
) where
    AssetsT: 'static,
    StateT: 'static,
{
    let event_loop = EventLoop::new();
    let (mut vulkan_config, mut vulkan_state) = vulkan_init(&event_loop, width, height);

    let mut now_keys = [false; 255];
    let mut prev_keys = [false; 255];
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
                        input:
                            winit::event::KeyboardInput {
                                virtual_keycode: Some(keycode),
                                state,
                                ..
                            },
                        ..
                    },
                ..
            } => match state {
                winit::event::ElementState::Pressed => {
                    now_keys[keycode as usize] = true;
                }
                winit::event::ElementState::Released => {
                    now_keys[keycode as usize] = false;
                }
            },
            Event::MainEventsCleared => {
                // ^-- input handling and vulkan junk

                update(&now_keys, &mut s, &a);
                render(&a, &s, &mut vulkan_config.fb2d);

                // now_keys are officially "old" now, after update
                prev_keys.copy_from_slice(&now_keys);

                render3d(&mut vulkan_config, &mut vulkan_state);
            }
            _ => (),
        }
    });
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
