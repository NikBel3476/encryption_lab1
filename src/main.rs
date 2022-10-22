use yew::{
    prelude::*,
    events::InputEvent
};
use web_sys::HtmlInputElement;
use wasm_logger;

pub mod encryption;

enum Msg {
    MessageInputChange(String)
}

struct Model {
    hash: String
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            hash: String::new()
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::MessageInputChange(message) => self.hash = encryption::encrypt(&message)
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        let on_message_input_change = link.batch_callback(|e: InputEvent| {
            let input = e.target_dyn_into::<HtmlInputElement>();

            input.map(|input| Msg::MessageInputChange(input.value()))
        });

        html! {
            <form class="message-form">
                <label for="message">{"Сообщение"}</label>
                <input
                    type="text"
                    class="message-input"
                    id="message"
                    name="message"
                    oninput={on_message_input_change}
                    pattern=r"^[a-zA-Z0-9]+$"
                />
                <span class="invalid-message-label">{"Можно ввести только символы латинского алфавита и цифры"}</span>
                <p>{"Шифр: "}{ &self.hash }</p>
            </form>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Model>();
}
