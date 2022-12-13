
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
// FIXME: This allow will eventually cancel an error, eventually remove it.
#![allow(unaligned_references)]
#![allow(unused_imports)]
#![no_std]
#![feature(start)]

// Make sure the allocator is set.
extern crate alloc;
use ogc_rs::prelude::*;
use ogc_rs::input::*;
use num::traits::float::FloatCore;
use micromath::F32Ext;

use hecs::*;

pub mod renderer;
use renderer::*;

pub mod modulator;
use modulator::*;
use modulator::sources::*;

mod game;
pub use game::*;


use linkme::distributed_slice;

#[derive(Debug)]
pub struct IntegrationTest {
    pub name: &'static str,
    pub test_fn: fn(),
}



#[distributed_slice]
pub static BENCHMARKS: [fn(&mut Bencher)] = [..];



fn basic_test() {
    println!("Running basic test");
}

inventory::submit!(IntegrationTest {
    name: "basic",
    test_fn: basic_test
});


inventory::collect!(IntegrationTest);


fn currenttime() -> u64 {
    // let mut timestruct = core::mem::MaybeUninit::uninit();
    // unsafe { clock_gettime(1, timestruct.as_mut_ptr()) };
    // let timestruct = unsafe { timestruct.assume_init() };
    // println!("timestruct: {:?}", timestruct);
    // 0
    // (timestruct.tv_sec as u64) * 1000000 + (timestruct.tv_nsec as u64)
    let val = ogc_rs::system::System::system_time();
    println!("time: {:?}", val);
    val

}

#[start]
/**
 * Main entrypoint of the application.
 */
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    println!("Hello Rust!");
    if cfg!(feature="run_integration_tests") {
        println!("Running the integration test suite...");
        let coll = vec![IntegrationTest {
            name: "basic",
            test_fn: basic_test
}];
        // for t in inventory::iter::<IntegrationTest> {
        for t in coll.iter() {
            println!("Running test {}...", t.name);
            (t.test_fn)()
        }
        println!("Done.");
        return 1;
    }

    let mut modenv : ModulatorEnv<f32> = Default::default();

    {
        let mut sine_wave_modulator = Wave::new(2.0, 0.5);
        sine_wave_modulator.set_enabled(true);
        sine_wave_modulator.set_wave(Box::new(|w, t| {
            (t * w.frequency * core::f32::consts::PI * 2.0).sin() * w.amplitude
        }));

        modenv.take("myfancywave", Box::new(sine_wave_modulator)); // start with 2.0 amplitude and 0.5Hz frequency)
    }


    // Setup the wiimote
    Input::init(ControllerType::Wii);
    let wii_mote = Input::new(ControllerType::Wii, ControllerPort::One);
    wii_mote
    .as_wpad()
    .set_data_format(WPadDataFormat::ButtonsAccelIR);

    // Setup the ECS environment.
    let mut world = World::new();
    batch_spawn_entities(&mut world, 5);
    let mut velocity_query = PreparedQuery::<&mut Velocity>::default();
    let mut all_query = PreparedQuery::<(&mut Position, &mut Velocity)>::default();

    // Kickstart main loop.
    let mut renderer = Renderer::new();
    renderer.init_render();

    
    //let mut last_frame_time = gettime(); // ogc_rs::system::System::system_time();
    let mut prev_frame_ns = currenttime();
    loop {
        let current_frame_ns = currenttime();
        let delta_time_ns = 1000/15*1000; // current_frame_ns - prev_frame_ns;

        Input::update(ControllerType::Wii);
        if wii_mote.is_button_down(Button::Home) {
            break
        }
        if wii_mote.is_button_down(Button::One) {
            system_shake_wii(&mut world, &mut velocity_query);
        }
        system_bounce_bounds(&mut world, &mut all_query);
        system_integrate_motion(&mut world, &mut all_query);

        modenv.advance((delta_time_ns) as u64);
        println!("delta time: {}", delta_time_ns);
        println!("Wave modulator value: {}", modenv.get("myfancywave").unwrap().value());

        renderer.render_world(&world);
        Video::flush();
        Video::wait_vsync();

        prev_frame_ns = current_frame_ns;
        // last_frame_time = gettime(); // ogc_rs::system::System::system_time();
    }
    renderer.close_render();
    return 0;
}
