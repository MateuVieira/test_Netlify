use dioxus::prelude::*;

// const _STYLE: &str = asset!("public/tailwind.css");

fn main() {
    wasm_logger::init(wasm_logger::Config::default()); // Initialize logging for the browser
    launch(app); 
}

fn app() -> Element {
    let mut name = use_signal(|| "".to_string());
    let mut greeting = use_signal( || "".to_string());

    rsx! {
        div {
            input {
                placeholder: "Enter your name",
                value: "{name}",
                oninput: move |evt| name.set(evt.value().clone())
            }
            button {
                onclick: move |_| {
                    // Make a fetch request to your Netlify function
                    let future = async move {
                        let response = reqwest::Client::new()
                            .post("/.netlify/functions/greet") // Replace with your function name
                            .json(&serde_json::json!({ "name": *name.read() }))
                            .send()
                            .await;

                            match response {
                                Ok(response) => {
                                    let text = response.text().await.unwrap_or_else(|_| "Error fetching greeting".to_string());
                                    greeting.set(text);
                                }
                                Err(err) => {
                                    // Handle the error gracefully (e.g., log it or display an error message)
                                    log::error!("Error fetching greeting: {:?}", err);
                                    greeting.set("Error fetching greeting".to_string());
                                }
                            }
                    };

                    spawn(future);
                },
                "Greet me!"
            }
            p { "{greeting}" }
        }
    }
}
