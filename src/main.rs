use yew::{
    prelude::*,
    events::InputEvent
};
use web_sys::HtmlInputElement;
use regex::Regex;
use wasm_logger;

pub mod encryption;

enum Msg {
    SecretKeyChange(String),
    MessageInputChange(String),
    HashInputChange(String)
}

struct Model {
    secret_key: String,
    message_input: String,
    message_hash: String,
    hash_input: String,
    encoded_message: String
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            secret_key: String::from("SECRET"),
            message_input: String::new(),
            message_hash: String::new(),
            hash_input: String::new(),
            encoded_message: String::new()
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SecretKeyChange(secret_key) => {
                let re = Regex::new(r"^[a-zA-Z0-9]+$").unwrap();
                let re_hash = Regex::new(r"^[a-zA-Z0-9\s]+$").unwrap();
                match secret_key.len() > 0 && re.is_match(&secret_key) {
                    true => {
                        self.secret_key = secret_key;
                        if re.is_match(&self.message_input) {
                            self.message_hash = encryption::encrypt(&self.message_input, &self.secret_key);
                        }
                        if re_hash.is_match(&self.hash_input) {
                            self.encoded_message = encryption::decrypt(&self.hash_input, &self.secret_key)
                        }
                    },
                    _ => ()
                }
            }
            Msg::MessageInputChange(message) => {
                let re = Regex::new(r"^[a-zA-Z0-9]+$").unwrap();
                match re.is_match(&message) {
                    true => {
                        self.message_input = message.clone();
                        self.message_hash = encryption::encrypt(&message, &self.secret_key);
                    },
                    false => {
                        self.message_input = String::new();
                        self.message_hash = String::new();
                    }
                }
            },
            Msg::HashInputChange(hash) => {
                let re = Regex::new(r"^[a-zA-Z0-9\s]+$").unwrap();
                match re.is_match(&hash) {
                    true => {
                        self.hash_input = hash.clone();
                        self.encoded_message = encryption::decrypt(&hash, &self.secret_key);
                    },
                    false => {
                        self.hash_input = String::new();
                        self.encoded_message = String::new();
                    }
                }
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        let on_secret_key_input_change = link.batch_callback(|e: InputEvent| {
            let input = e.target_dyn_into::<HtmlInputElement>();
            input.map(|input| Msg::SecretKeyChange(input.value()))
        });

        let on_message_input_change = link.batch_callback(|e: InputEvent| {
            let input = e.target_dyn_into::<HtmlInputElement>();
            input.map(|input| Msg::MessageInputChange(input.value()))
        });

        let on_hash_input_change = link.batch_callback(|e: InputEvent| {
            let input = e.target_dyn_into::<HtmlInputElement>();
            input.map(|input| Msg::HashInputChange(input.value()))
        });

        html! {
            <main>
            <form class="message-form">
                <label for="secret-key">{"???????? ????????????????????"}</label>
                <input
                    type="text"
                    class="secret-key-input"
                    id="secret-key"
                    name="secret-key"
                    minlength="1"
                    placeholder="SECRET"
                    oninput={on_secret_key_input_change}
                    pattern=r"^[a-zA-Z0-9]+$"
                />
                <span class="invalid-secret-key-label">
                    {"???????? ???????????? ?????????????????? ???????????? ?????????? ???????????????????? ???????????????? ?????? ??????????"}
                </span>
                <h3 class="form-title">{ "????????????????????" }</h3>
                <label for="message">{ "??????????????????" }</label>
                <input
                    type="text"
                    class="message-input"
                    id="message"
                    name="message"
                    oninput={on_message_input_change}
                    pattern=r"^[a-zA-Z0-9]+$"
                />
                <span class="invalid-message-label">{"?????????? ???????????? ???????????? ?????????????? ???????????????????? ???????????????? ?? ??????????"}</span>
                <p class="form-output__label">{ "????????:" }</p>
                <p class="form-output">{ &self.message_hash }</p>
            </form>
            <form class="hash-form">
                <h3 class="form-title">{ "??????????????????????" }</h3>
                <label for="hash">{"????????"}</label>
                <input
                    type="text"
                    class="hash-input"
                    id="hash"
                    name="hash"
                    oninput={on_hash_input_change}
                    pattern=r"^[a-zA-Z0-9\s]+$"
                />
                <span class="invalid-hash-label">{"?????????? ???????????? ???????????? ?????????????? ???????????????????? ????????????????, ?????????? ?? ??????????????"}</span>
                <p class="form-output__label">{ "??????????????????:" }</p>
                <p class="form-output">{ &self.encoded_message }</p>
            </form>
            <table class="encryption-table">
                <thead>
                    <tr>
                        <th colspan="7">{"?????????????? ????????????????????"}</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>{" "}</td>
                        <td scope="row">{"A"}</td>
                        <td scope="row">{"B"}</td>
                        <td scope="row">{"C"}</td>
                        <td scope="row">{"D"}</td>
                        <td scope="row">{"E"}</td>
                        <td scope="row">{"F"}</td>
                    </tr>
                    <tr>
                        <td scope="col">{"A"}</td>
                        <td>{"A"}</td>
                        <td>{"B"}</td>
                        <td>{"C"}</td>
                        <td>{"D"}</td>
                        <td>{"E"}</td>
                        <td>{"F"}</td>
                    </tr>
                    <tr>
                        <td scope="col">{"B"}</td>
                        <td>{"G"}</td>
                        <td>{"H"}</td>
                        <td>{"I"}</td>
                        <td>{"J"}</td>
                        <td>{"K"}</td>
                        <td>{"L"}</td>
                    </tr>
                    <tr>
                        <td scope="col">{"C"}</td>
                        <td>{"M"}</td>
                        <td>{"N"}</td>
                        <td>{"O"}</td>
                        <td>{"P"}</td>
                        <td>{"Q"}</td>
                        <td>{"R"}</td>
                    </tr>
                    <tr>
                        <td scope="col">{"D"}</td>
                        <td>{"S"}</td>
                        <td>{"T"}</td>
                        <td>{"U"}</td>
                        <td>{"V"}</td>
                        <td>{"W"}</td>
                        <td>{"X"}</td>
                    </tr>
                    <tr>
                        <td scope="col">{"E"}</td>
                        <td>{"Y"}</td>
                        <td>{"Z"}</td>
                        <td>{"0"}</td>
                        <td>{"1"}</td>
                        <td>{"2"}</td>
                        <td>{"3"}</td>
                    </tr>
                    <tr>
                        <td scope="col">{"F"}</td>
                        <td>{"4"}</td>
                        <td>{"5"}</td>
                        <td>{"6"}</td>
                        <td>{"7"}</td>
                        <td>{"8"}</td>
                        <td>{"9"}</td>
                    </tr>
                </tbody>
            </table>
            </main>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Model>();
}
