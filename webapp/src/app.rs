use std::sync::Arc;

use raylib::random_scene;
use raylib::Raytracer;
use raylib::RaytracerOptions;
use wasm_bindgen::Clamped;
use wasm_bindgen::JsCast;
use yew::prelude::*;
use yew::NodeRef;

pub struct Greet {
    title: String,
    msg: String,
    canvas_ref: NodeRef,
    image: Vec<u8>,
}

const WIDTH: u32 = 300;
const HEIGHT: u32 = 200;
const ASPECT_RATIO: f64 = WIDTH as f64 / HEIGHT as f64;

impl Component for Greet {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        raylib::hello_raylib();
        let world = Arc::new(random_scene());
        let raytracer = Raytracer::new(
            world,
            &RaytracerOptions {
                image_width: WIDTH,
                aspect_ratio: ASPECT_RATIO,
                max_depth: 5,
                samples_per_pixel: 2,
            },
        );

        let mut image = raylib::Image::new(WIDTH, HEIGHT);
        (0..HEIGHT).into_iter().for_each(|line_number| {
            let line = raytracer.trace_line(line_number);
            image.set_line(HEIGHT - line_number - 1, line);
            log::info!("finished line {}", line_number);
        });

        Self {
            title: "Raytracing".to_string(),
            msg: "Output".to_string(),
            canvas_ref: NodeRef::default(),
            image: image.buf().clone(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <h1>{self.title.clone()}</h1>
                <p>{self.msg.clone()}</p>
                <canvas width={WIDTH.to_string()} height={HEIGHT.to_string()} ref={self.canvas_ref.clone()}></canvas>
            </div>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {
        let canvas = self
            .canvas_ref
            .cast::<web_sys::HtmlCanvasElement>()
            .unwrap();

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        let image_data =
            web_sys::ImageData::new_with_u8_clamped_array(Clamped(&self.image), WIDTH).unwrap();

        context.put_image_data(&image_data, 0.0, 0.0);
    }
}
