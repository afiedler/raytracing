mod utils;

use std::sync::{Arc, Mutex};

use async_channel::{RecvError, TryRecvError};
use js_sys::{Uint8Array, Uint8ClampedArray};
use log::logger;
use raylib::{random_scene, Image, Rand, Raytracer, RaytracerOptions};
use wasm_bindgen::{prelude::*, Clamped, *};
use web_sys::DedicatedWorkerGlobalScope;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[cfg(target_arch = "wasm32")]
mod wasm {
    use wasm_bindgen::prelude::*;

    // Prevent `wasm_bindgen` from autostarting main on all spawned threads
    #[wasm_bindgen(start)]
    pub fn dummy_main() {}

    // Export explicit run function to start main
    #[wasm_bindgen]
    pub fn run() {
        // console_log::init().unwrap();
        wasm_logger::init(wasm_logger::Config::default());
        console_error_panic_hook::set_once();
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn raytrace() {
    log::info!("raytracing...");
    let width = 1200;
    let height = 800;
    let aspect_ratio = width as f64 / height as f64;
    let mut rand = Rand::new();
    let scene = random_scene(&mut rand);
    let image_mutex = Mutex::new(Image::new(width, height));
    let raytracer = Arc::new(Raytracer::new(
        scene,
        &RaytracerOptions {
            image_width: width,
            aspect_ratio,
            max_depth: 50,
            samples_per_pixel: 1,
        },
    ));

    let mut height_iter = (0..height).into_iter();
    let height_iter_mutex = Arc::new(Mutex::new(height_iter));

    let (tx, mut rx) = async_channel::unbounded();

    for t in 0..4 {
        let raytracer_c = raytracer.clone();
        let tx_c = tx.clone();
        let height_iter_mutex_c = height_iter_mutex.clone();
        wasm_thread::spawn(move || {
            while let Some(line_number) = {
                let mut height_iter = height_iter_mutex_c.lock().unwrap();
                height_iter.next()
            } {
                let mut rand_seed: [u8; 16] = [0; 16];
                getrandom::getrandom(&mut rand_seed);
                let mut rand = Rand::new_with_seed(u128::from_le_bytes(rand_seed));
                let line = raytracer_c.trace_line(height - 1 - line_number, &mut rand);
                tx_c.try_send((line_number, line)).unwrap();
            }
        });
    }

    wasm_bindgen_futures::spawn_local(async move {
        while let msg = rx.recv().await {
            match msg {
                Ok(msg) => {
                    let obj = js_sys::Object::new();
                    js_sys::Reflect::set(&obj, &"type".into(), &"progress".into());
                    js_sys::Reflect::set(&obj, &"lineNumber".into(), &msg.0.into());

                    let mut line_data = Uint8ClampedArray::new_with_length(msg.1.len() as u32);
                    line_data.copy_from(&msg.1);

                    js_sys::Reflect::set(&obj, &"line".into(), &line_data);
                    js_sys::eval("self")
                        .unwrap()
                        .dyn_into::<DedicatedWorkerGlobalScope>()
                        .unwrap()
                        .post_message(&obj.into())
                        .unwrap();
                }
                Err(RecvError) => break,
            }
        }

        let obj = js_sys::Object::new();
        js_sys::Reflect::set(&obj, &"type".into(), &"finished".into());

        js_sys::eval("self")
            .unwrap()
            .dyn_into::<DedicatedWorkerGlobalScope>()
            .unwrap()
            .post_message(&obj.into())
            .unwrap();
    });
}
