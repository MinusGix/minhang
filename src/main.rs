use glazier::{WinHandler, WindowBuilder, WindowHandle};
use wgpu::{Backends, InstanceDescriptor};

struct Handler;
impl WinHandler for Handler {
    fn connect(&mut self, handle: &WindowHandle) {
        init(handle);
    }

    fn prepare_paint(&mut self) {}

    fn paint(&mut self, _: &glazier::Region) {}

    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

struct AppThing;
impl glazier::AppHandler for AppThing {
    fn command(&mut self, _: u32) {}
}

pub fn init(window: &WindowHandle) {
    println!("Init");
    // Seems to default to GL
    let instance = wgpu::Instance::default();
    // Manually requiring Vulkan makes surface creation works
    // let instance = wgpu::Instance::new(InstanceDescriptor {
    //     backends: Backends::VULKAN,
    //     ..Default::default()
    // });
    println!("Made WGPU Instance");

    // Stalls if instance is gl
    let surface = unsafe { instance.create_surface(window) }.unwrap();
    println!("Made WGPU Surface");

    let _adapter =
        futures::executor::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }))
        .unwrap();
    println!("Got adapter");

    todo!()
}

fn main() {
    env_logger::init();

    {
        let handler = Box::new(Handler);
        let app = glazier::Application::new().unwrap();

        let mut builder = WindowBuilder::new(app.clone());

        builder = builder.handler(handler);
        let window = builder.build().unwrap();
        window.show();

        let app_thing = AppThing;
        println!("Running");
        app.run(Some(Box::new(app_thing)));
    }
}
