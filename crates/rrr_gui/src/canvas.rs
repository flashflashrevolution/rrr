use std::fmt::Debug;

use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew_services::ConsoleService;

#[derive(Debug)]
pub enum Msg {
    Render,
}

#[derive(Debug)]
pub struct Radial {
    canvas: Option<HtmlCanvasElement>,
    canvas_node_ref: NodeRef,
    link: ComponentLink<Self>,
}

impl Component for Radial {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            canvas: None,
            canvas_node_ref: NodeRef::default(),
            link,
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
        let link = &self.link;
        html! {
            <canvas ref=self.canvas_node_ref.clone() />
        }
    }
}

impl Model {
    fn render(&mut self, timestamp: f64) {
        ConsoleService::log("click");
    }
}
