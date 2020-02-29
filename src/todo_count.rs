use yew::prelude::*;

pub struct TodoCount {
    link: ComponentLink<Self>,
    nb_todos: u32
}

pub enum Msg {
    Click
}

impl Component for TodoCount {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        TodoCount {
            link,
            nb_todos: 0
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => self.nb_todos = self.nb_todos + 1
        }
        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
                { &self.nb_todos.to_string() }
                <button onclick=&self.link.callback(|_| Msg::Click)>{ "increment" }</button>
            </div>
        }
    }
}
