use yew::prelude::*;

#[derive(Clone)]
pub struct TodoItem {
    link: ComponentLink<Self>,
    text: String,
    completed: bool,
}

#[derive(Clone, Properties)]
pub struct Props {
    pub text: String,
}

pub enum Msg {
    Click
}

impl Component for TodoItem {
    type Message = Msg;
    type Properties = Props;

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        TodoItem {
            link,
            text: String::from("test"),
            completed: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => {
                self.completed = !self.completed;
                true
            }
        }
    }

    fn view(&self) -> Html {
        let item_text = if self.completed { "completed" } else { "not done yet" };
        html! {
            <div>
                { &self.text }
                <button onclick=&self.link.callback(|_| Msg::Click)>{ item_text }</button>
            </div>
        }
    }
}
