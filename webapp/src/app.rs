use raylib::raytracer;
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

impl Component for Greet {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        raylib::hello_raylib();
        let image = raytracer();
        Self {
            title: "Raytracing".to_string(),
            msg: "Output".to_string(),
            canvas_ref: NodeRef::default(),
            image,
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
                <canvas width={"400"} height={"256"} ref={self.canvas_ref.clone()}></canvas>
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
            web_sys::ImageData::new_with_u8_clamped_array(Clamped(&self.image), 400).unwrap();

        context.put_image_data(&image_data, 0.0, 0.0);
    }
}
