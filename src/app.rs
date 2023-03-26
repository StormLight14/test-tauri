use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use tungstenite::{connect, Message};
use url::Url;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct FormData<'a> {
    username: &'a str,
    rating: &'a str,
    review: &'a str,
}

#[function_component(App)]
pub fn app() -> Html {
    let user_input_ref = use_node_ref();
    let rating_input_ref = use_node_ref();
    let review_input_ref = use_node_ref();

    let username = use_state(|| String::new());
    let rating = use_state(|| String::new());
    let review = use_state(|| String::new());

    let user_handler = {
        let username = username.clone();
        let user_input_ref = user_input_ref.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            username.set(
                user_input_ref
                    .cast::<web_sys::HtmlInputElement>()
                    .unwrap()
                    .value(),
            );
        })
    };

    let rating_handler = {
        let rating = rating.clone();
        let rating_input_ref = rating_input_ref.clone();
        Callback::from(move |e: SubmitEvent| {
            rating.set(
                rating_input_ref
                    .cast::<web_sys::HtmlInputElement>()
                    .unwrap()
                    .value(),
            );
        })
    };

    let review_handler = {
        let review = review.clone();
        let review_input_ref = review_input_ref.clone();
        Callback::from(move |e: SubmitEvent| {
            review.set(
                review_input_ref
                    .cast::<web_sys::HtmlInputElement>()
                    .unwrap()
                    .value(),
            );
        })
    };

    let submit_form = {
        let username = username.clone();
        let rating = rating.clone();
        let review = review.clone();
        let user_input_ref = user_input_ref.clone();
        let rating_input_ref = rating_input_ref.clone();
        let review_input_ref = review_input_ref.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            username.set(
                user_input_ref
                    .cast::<web_sys::HtmlInputElement>()
                    .unwrap()
                    .value(),
            );
            rating.set(
                rating_input_ref
                    .cast::<web_sys::HtmlInputElement>()
                    .unwrap()
                    .value(),
            );
            review.set(
                review_input_ref
                    .cast::<web_sys::HtmlInputElement>()
                    .unwrap()
                    .value(),
            );

            let form_data = FormData {
                username: &*username,
                rating: &*rating,
                review: &*review,
            };

            let args = to_value(&form_data).unwrap();
            invoke("submit_form", args);
            
            let message = Message::Text((format!("{:?},{:?},{:?}", &*username, &*rating, &*review)));

            env_logger::init();

            let (mut socket, response) = connect(Url::parse("ws://localhost:3012/socket").unwrap()).expect("Can't connect");

            socket.write_message(Message::Text(message.to_string())).unwrap();
            let msg = socket.read_message().expect("Error reading message");
            println!("Sent: {}", msg);
        })
    };

    html! {
        <main class="container">
            <div class="row">
                <a href="https://www.youtube.com/watch?v=dQw4w9WgXcQ" target="_blank">
                    <img src="public/tauri.svg" class="logo tauri" alt="Tauri logo"/>
                </a>
                <a href="https://www.youtube.com/watch?v=dQw4w9WgXcQ" target="_blank">
                    <img src="public/yew.png" class="logo yew" alt="Yew logo"/>
                </a>
            </div>

            <p>{"Here's the time"}</p>

            <p>
                {"clock here"}
            </p>

            <form class="row" onsubmit={submit_form}>
                <input id="username-input" ref={user_input_ref} placeholder="Enter username..." />
                <input id="rating-input" ref={rating_input_ref} placeholder="Enter rating..." />
                <input id="review-input" ref={review_input_ref} placeholder="Enter review..." />
                <button type="submit">{"Submit"}</button>
            </form>
            <p>{format!("Username: {:?} Rating: {:?} Review: {:?}", *username, *rating, *review)}</p>
        </main>
    }
}
