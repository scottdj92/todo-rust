use yew::prelude::*;
mod todo_item;
mod todo_count;

struct App {
    clicked: bool,
}

enum Msg {
    Click,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            clicked: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => {
                self.clicked = true;
                true
            }
        }
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <todo_item::TodoItem text=String::from("test") />
                <todo_count::TodoCount />
            </div>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
