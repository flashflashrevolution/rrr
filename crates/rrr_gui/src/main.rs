mod canvas;
mod radial;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub enum Msg {
    SpitMessage,
}

pub struct Model {
    _link: ComponentLink<Self>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { _link }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div id="game_root" class="root gradient">
                <radial::Radial/>
                <canvas::Canvas/>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
