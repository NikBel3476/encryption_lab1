use yew::{
    prelude::*,
    events::InputEvent
};
use web_sys::HtmlInputElement;
use regex::Regex;
use wasm_logger;

pub mod encryption;

enum Msg {
    MessageInputChange(String),
    HashInputChange(String)
}

struct Model {
    message_hash: String,
    encoded_message: String
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            message_hash: String::new(),
            encoded_message: String::new()
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::MessageInputChange(message) => {
                let re = Regex::new(r"^[a-zA-Z0-9]+$").unwrap();
                match re.is_match(&message) {
                    true => self.message_hash = {
                        let hash = encryption::encrypt(&message);
                        log::info!("{}", &hash);
                        hash
                    },
                    false => self.message_hash = String::new()
                }
            },
            Msg::HashInputChange(hash) => {
                let re = Regex::new(r"^[a-zA-Z0-9\s]+$").unwrap();
                match re.is_match(&hash) {
                    true => self.encoded_message = encryption::decrypt(&hash),
                    false => self.encoded_message = String::new()
                }
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        let on_message_input_change = link.batch_callback(|e: InputEvent| {
            let input = e.target_dyn_into::<HtmlInputElement>();
            input.map(|input| Msg::MessageInputChange(input.value()))

        });

        let on_hash_input_change = link.batch_callback(|e: InputEvent| {
            let input = e.target_dyn_into::<HtmlInputElement>();
            input.map(|input| Msg::HashInputChange(input.value()))
        });

        html! {
            <>
            <form class="message-form">
                <h3 class="form-title">{ "Шифрование" }</h3>
                <label for="message">{ "Сообщение" }</label>
                <input
                    type="text"
                    class="message-input"
                    id="message"
                    name="message"
                    oninput={on_message_input_change}
                    pattern=r"^[a-zA-Z0-9]+$"
                />
                <span class="invalid-message-label">{"Можно ввести только символы латинского алфавита и цифры"}</span>
                <p class="form-output">{ "Шифр:" }<pre>{ &self.message_hash }</pre></p>
            </form>
            <form class="hash-form">
                <h3 class="form-title">{ "Расшифровка" }</h3>
                <label for="hash">{"Шифр"}</label>
                <input
                    type="text"
                    class="hash-input"
                    id="hash"
                    name="hash"
                    oninput={on_hash_input_change}
                    pattern=r"^[a-zA-Z0-9\s]+$"
                />
                <span class="invalid-hash-label">{"Можно ввести только символы латинского алфавита, цифры и пробелы"}</span>
                <p class="form-output">{ "Сообщение:" }<pre>{ &self.encoded_message }</pre></p>
            </form>
            </>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Model>();
}
