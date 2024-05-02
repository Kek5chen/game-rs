use wgpu::{include_wgsl, Backends, ColorTargetState, ColorWrites, FragmentState, RenderPipeline, RenderPipelineDescriptor, VertexState, Adapter, Queue, Device, Surface, SurfaceConfiguration, ShaderModule};
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use winit::window::Window;

pub struct State {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    window: Window,
    pipeline: RenderPipeline,
    color: wgpu::Color,
}

impl State {
    fn setup_instance() -> wgpu::Instance {
        let instance = wgpu::Instance::default();

        print!("Available Graphics Units: ");
        let backends = instance
            .enumerate_adapters(Backends::all())
            .iter()
            .map(|a| format!("{} ({})", a.get_info().name, a.get_info().backend.to_str()))
            .collect::<Vec<String>>()
            .join(", ");
        println!("{}", backends);

        instance
    }
    fn setup_surface(instance: &wgpu::Instance, window: &Window) -> wgpu::Surface<'static> {
        let surface = unsafe {
            // We are creating a 'static lifetime out of a local reference
            // VERY UNSAFE: Make absolutely sure `window` lives as long as `surface`
            let surface = instance.create_surface(window).unwrap();
            std::mem::transmute::<wgpu::Surface, wgpu::Surface<'static>>(surface)
        };

        surface
    }

    async fn setup_adapter(instance: &wgpu::Instance, surface: &wgpu::Surface<'_>) -> Adapter {
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(surface),
                ..Default::default()
            })
            .await
            .expect(
                "Couldn't find anything that supports rendering stuff. How are you reading this..?",
            );

        println!("Using: {} through {}", adapter.get_info().name, adapter.get_info().backend.to_str());
        adapter
    }

    async fn get_device_and_queue(adapter: &Adapter) -> (Device, Queue) {
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default(), None)
            .await
            .unwrap();
        (device, queue)
    }

    fn configure_surface(size: &PhysicalSize<u32>, surface: &Surface, adapter: &Adapter, device: &Device) -> SurfaceConfiguration {
        let config = surface
            .get_default_config(adapter, size.width, size.height)
            .unwrap();
        surface.configure(device, &config);
        config
    }

    fn setup_pipeline(device: &Device, config: &SurfaceConfiguration, shader: &ShaderModule) -> RenderPipeline {
        let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: None,
            vertex: VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[],
            },
            primitive: Default::default(),
            depth_stencil: None,
            multisample: Default::default(),
            fragment: Some(FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(ColorTargetState {
                    format: config.format,
                    blend: None,
                    write_mask: ColorWrites::ALL,
                })],
            }),
            multiview: None,
        });
        pipeline
    }

    fn load_shader(device: &Device) -> ShaderModule {
        let shader = device.create_shader_module(include_wgsl!("shader.wgsl"));
        println!("Loaded `shader.wgsl`..");
        shader
    }

    pub async fn new(window: Window) -> Self {
        let size = window.inner_size();
        let size = PhysicalSize {
            height: size.height.min(1),
            width: size.width.min(1),
        };

        let instance = Self::setup_instance();
        let surface = Self::setup_surface(&instance, &window);
        let adapter = Self::setup_adapter(&instance, &surface).await;
        let (device, queue) = Self::get_device_and_queue(&adapter).await;
        let config = Self::configure_surface(&size, &surface, &adapter, &device);

        let shader = Self::load_shader(&device);
        let pipeline = Self::setup_pipeline(&device, &config, &shader);

        State {
            surface,
            device,
            queue,
            config,
            size,
            window,
            pipeline,
            color: wgpu::Color {
                r: 0.1,
                g: 0.2,
                b: 0.3,
                a: 1.0,
            },
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn resize(&mut self, mut new_size: PhysicalSize<u32>) {
        new_size.height = new_size.height.max(1);
        new_size.width = new_size.width.max(1);
        self.size = new_size;
        self.config.width = new_size.width;
        self.config.height = new_size.height;
        self.surface.configure(&self.device, &self.config);
    }

    pub fn update(&mut self) {
        // TODO
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(self.color),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
            rpass.set_pipeline(&self.pipeline);
            rpass.draw(0..3, 0..1)
        }

        self.queue.submit(Some(encoder.finish()));
        output.present();

        Ok(())
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::CursorMoved { position, .. } => {
                self.color = wgpu::Color {
                    r: position.x / self.size.width as f64,
                    g: position.y / self.size.height as f64,
                    b: (position.x + 1.0) / 2.0 / self.size.width as f64,
                    a: 1.0,
                };
                println!("mow: {:?}", self.color);
                self.window.request_redraw();
                true
            }
            _ => false,
        }
    }
}
