use std::fmt::Debug;

use web_sys::HtmlCanvasElement;
use yew::{html, Component, ComponentLink, Html, NodeRef, ShouldRender};
use yew_services::{render::RenderTask, ConsoleService, RenderService};

#[derive(Debug)]
pub enum Msg {
    Render(f64),
}

#[derive(Debug)]
pub struct Canvas {
    canvas: Option<HtmlCanvasElement>,
    canvas_node_ref: NodeRef,
    link: ComponentLink<Self>,
    render_loop: Option<RenderTask>,
}

impl Component for Canvas {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            canvas: None,
            canvas_node_ref: NodeRef::default(),
            link,
            render_loop: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Render(timestamp) => {
                self.render(timestamp);
                false
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <canvas
                ref=self.canvas_node_ref.clone()
                id="rrr_canvas"
                class="rrr_canvas"/>
        }
    }
}

impl Canvas {
    fn render(&mut self, timestamp: f64) {
        ConsoleService::log(format!("click {}", timestamp).as_str());

        let render_frame = self.link.callback(Msg::Render);
        let handle = RenderService::request_animation_frame(render_frame);

        // A reference to the new handle must be retained for the next render to run.
        self.render_loop = Some(handle);
    }
}
