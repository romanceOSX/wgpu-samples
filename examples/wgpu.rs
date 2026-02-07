use log::info;
use wgpu::{Instance};

fn run() {

    //
    // Instance, Adapter and Device
    //

    let instance = Instance::default();
    println!("Enumerating available adapters:");
    for a in instance.enumerate_adapters(wgpu::Backends::PRIMARY) {
        println!("- {:?}", a.get_info());
    }

    let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptionsBase {
        power_preference: wgpu::PowerPreference::HighPerformance,
        force_fallback_adapter: false,
        compatible_surface: None, 
    })).unwrap();

    println!("- Selected Adapter:\n{:?}", adapter.get_info());

    let (dev, queue) = pollster::block_on(
        adapter.request_device(&wgpu::DeviceDescriptor::default())
    ).unwrap();

    //
    //
    //

}

fn stub() {
    env_logger::init();

    let instance = Instance::new(&wgpu::InstanceDescriptor {
        backends: wgpu::Backends::PRIMARY,
        ..Default::default()
    });

    let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions{
        power_preference: wgpu::PowerPreference::default(),
        compatible_surface: None,       // no surface :)
        force_fallback_adapter: false,
    })).unwrap();

    // print features
    info!("Printing devices info...");
    for f in adapter.features().iter() {
        info!("{f:?}");
    }

    // device and queue
    let (device, queue) = pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor {
        label: None,
        required_features: wgpu::Features::empty(),
        experimental_features: wgpu::ExperimentalFeatures::disabled(),
        required_limits: wgpu::Limits::defaults(),
        memory_hints: Default::default(),
        trace: wgpu::Trace::Off,
    })).unwrap();
}

fn main() {
    run();
}

