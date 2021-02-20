use yew::{
    html,
    web_sys::{Document, Element},
    App, Component, ComponentLink, Html, ShouldRender,
};

pub enum Msg {
    SpitMessage,
}

pub struct Model {
    link: ComponentLink<Self>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <button onclick=&self.link.callback(|_| Msg::SpitMessage)>{ "word" }</button>
        }
    }
}

fn main() {
    let category_app: App<Model> = App::new();
    let song_selection_app: App<Model> = App::new();
    let document: Document = yew::utils::document();

    let category_root: Element = document
        .get_element_by_id("category_selection_root")
        .unwrap();

    let song_selection_root: Element = document.get_element_by_id("song_selection_root").unwrap();

    // Design mode for live designing in HTML?
    category_app.mount(category_root);
    song_selection_app.mount(song_selection_root);
}
