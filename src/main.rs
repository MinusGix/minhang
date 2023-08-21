use glazier::{
    kurbo::{Point, Size},
    AppHandle, Scale, WinHandler, WindowBuilder, WindowHandle,
};
use wgpu::{Backends, InstanceDescriptor};

struct Handler {
    handle: WindowHandle,
}
impl WinHandler for Handler {
    fn connect(&mut self, handle: &WindowHandle) {
        self.handle = handle.clone();
        init(handle);
    }

    fn prepare_paint(&mut self) {}

    fn paint(&mut self, invalid: &glazier::Region) {}

    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

struct AppThing {}
impl glazier::AppHandler for AppThing {
    fn command(&mut self, id: u32) {}
}

pub fn init(window: &WindowHandle) {
    println!("Init");
    // Seems to default to GL
    // let instance = wgpu::Instance::default();
    // Manually requiring Vulkan makes surface creation works
    let instance = wgpu::Instance::new(InstanceDescriptor {
        backends: Backends::VULKAN,
        ..Default::default()
    });
    println!("Made WGPU Instance");

    // Stalls if instance is gl
    let surface = unsafe { instance.create_surface(window) }.unwrap();
    println!("Made WGPU Surface");

    let adapter =
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
        let app = glazier::Application::new().unwrap();
        println!("Made app");
    }

    // ...

    {
        let handler = Box::new(Handler {
            handle: WindowHandle::default(),
        });
        let app = glazier::Application::global();
        println!("Got global app");

        let mut builder = WindowBuilder::new(app.clone());
        // TODO: titlebar stuff? position? size?

        builder = builder.handler(handler);
        println!(" Set handler");
        let window = builder.build().unwrap();
        println!("Made window..");
        window.show();

        let app_thing = AppThing {};
        println!("Running");
        app.run(Some(Box::new(app_thing)));
    }
}
