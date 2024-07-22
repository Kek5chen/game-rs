use std::rc::Rc;
use wgpu::{
    Adapter

    , Device, DeviceDescriptor, Extent3d, Instance
    , PowerPreference, Queue,
    RequestAdapterOptions, Surface, SurfaceConfiguration,
    Texture, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages
    ,
};
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use winit::window::Window;

pub struct State {
    pub(crate) instance: Instance,
    pub(crate) surface: Surface<'static>,
    pub(crate) device: Rc<Device>,
    pub(crate) queue: Rc<Queue>,
    pub(crate) config: SurfaceConfiguration,
    pub(crate) size: PhysicalSize<u32>,
    pub(crate) depth_texture: Texture,
}

impl State {
    fn setup_instance() -> Instance {
        let instance = Instance::default();
        instance
    }
    fn setup_surface(instance: &Instance, window: &Window) -> Surface<'static> {
        let surface = unsafe {
            // We are creating a 'static lifetime out of a local reference
            // VERY UNSAFE: Make absolutely sure `window` lives as long as `surface`
            let surface = instance.create_surface(window).unwrap();
            std::mem::transmute::<Surface, Surface<'static>>(surface)
        };

        surface
    }

    async fn setup_adapter(instance: &Instance, surface: &Surface<'_>) -> Adapter {
        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::HighPerformance,
                compatible_surface: Some(surface),
                ..Default::default()
            })
            .await
            .expect(
                "Couldn't find anything that supports rendering stuff. How are you reading this..?",
            );
        adapter
    }

    async fn get_device_and_queue(adapter: &Adapter) -> (Rc<Device>, Rc<Queue>) {
        let (device, queue) = adapter
            .request_device(&DeviceDescriptor::default(), None)
            .await
            .unwrap();
        (Rc::new(device), Rc::new(queue))
    }

    fn configure_surface(
        size: &PhysicalSize<u32>,
        surface: &Surface,
        adapter: &Adapter,
        device: &Device,
    ) -> SurfaceConfiguration {
        let config = surface
            .get_default_config(adapter, size.width, size.height)
            .unwrap();
        surface.configure(device, &config);
        config
    }

    fn setup_depth_texture(size: &PhysicalSize<u32>, device: &Device) -> Texture {
        let depth_texture = device.create_texture(&TextureDescriptor {
            label: Some("Depth Texture"),
            size: Extent3d {
                width: size.width,
                height: size.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Depth32Float,
            usage: TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[TextureFormat::Depth32Float],
        });
        depth_texture
    }

    pub async fn new(window: &Window) -> Self {
        let size = window.inner_size();
        let size = PhysicalSize {
            height: size.height.max(1),
            width: size.width.max(1),
        };

        let instance = Self::setup_instance();
        let surface = Self::setup_surface(&instance, &window);
        let adapter = Self::setup_adapter(&instance, &surface).await;
        let (device, queue) = Self::get_device_and_queue(&adapter).await;
        let config = Self::configure_surface(&size, &surface, &adapter, &device);

        let depth_texture = Self::setup_depth_texture(&size, &device);

        State {
            instance,
            surface,
            device,
            queue,
            config,
            size,
            depth_texture,
        }
    }

    pub fn resize(&mut self, mut new_size: PhysicalSize<u32>) {
        new_size.height = new_size.height.max(1);
        new_size.width = new_size.width.max(1);
        self.size = new_size;
        self.config.width = new_size.width;
        self.config.height = new_size.height;
        self.surface.configure(&self.device, &self.config);
        self.depth_texture = Self::setup_depth_texture(&self.size, &self.device);
    }

    pub fn update(&mut self) {
        // TODO
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            _ => false,
        }
    }
}
