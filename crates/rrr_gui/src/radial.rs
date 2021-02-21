use std::fmt::Debug;

use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew_services::ConsoleService;

#[derive(Debug)]
pub enum Msg {
    Display,
}

#[derive(Debug)]
pub struct Radial {
    link: ComponentLink<Self>,
}

impl Component for Radial {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Display => {
                ConsoleService::log("click");
            }
        }
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let link = &self.link;
        html! {
            <div id="container" class="container" style="--m: 6; --tan: 0.60">
                <i>
                <div style="--i: 1">
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
                    <path d="M18 2l6 10.5-6 10.5h-12l-6-10.5 6-10.5z" onclick=link.callback(|_| Msg::Display)/>
                    </svg>
                </div>
                <div style="--i: 2">
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
                    <path d="M18 2l6 10.5-6 10.5h-12l-6-10.5 6-10.5z" onclick=link.callback(|_| Msg::Display)/>
                    </svg>
                </div>
                <div style="--i: 3">
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
                    <path d="M18 2l6 10.5-6 10.5h-12l-6-10.5 6-10.5z" onclick=link.callback(|_| Msg::Display)/>
                    </svg>
                </div>
                <div style="--i: 4">
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
                    <path d="M18 2l6 10.5-6 10.5h-12l-6-10.5 6-10.5z" onclick=link.callback(|_| Msg::Display)/>
                    </svg>
                </div>
                <div style="--i: 5">
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
                    <path d="M18 2l6 10.5-6 10.5h-12l-6-10.5 6-10.5z" onclick=link.callback(|_| Msg::Display)/>
                    </svg>
                </div>
                <div style="--i: 6">
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
                    <path d="M18 2l6 10.5-6 10.5h-12l-6-10.5 6-10.5z" onclick=link.callback(|_| Msg::Display)/>
                    </svg>
                </div>
                </i>
            </div>
        }
    }
}
