use dioxus::prelude::*;
use std::collections::HashMap;

fn main() {
    dioxus_desktop::launch(|cx| render!(app_root {}));
}

fn app_root(cx: Scope<'_>) -> Element {
    let breed = use_state(cx, || "deerhound".to_string());

    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    struct ListBreeds {
        message: HashMap<String, Vec<String>>,
    }

    let breeds = use_future!(cx, || async move {
        reqwest::get("https://dog.ceo/api/breeds/list/all")
            .await
            .unwrap()
            .json::<ListBreeds>()
            .await
    });

    match breeds.value()? {
        Ok(breeds) => cx.render(rsx! {
            div { height: "500px",
                h1 { "Select a dog breed!" }
                div { display: "flex",
                    ul { flex: "50%",
                        for cur_breed in breeds.message.keys().take(10) {
                            li { key: "{cur_breed}",
                                button {
                                    onclick: move |_| breed.set(cur_breed.clone()),
                                    "{cur_breed}"
                                }
                            }
                        }
                    }
                    div { flex: "50%", breed_pic { breed: breed.to_string() } }
                }
            }
        }),
        Err(_e) => cx.render(rsx! { div { "Error fetching breeds" } }),
    }
}

#[inline_props]
fn breed_pic(cx: Scope, breed: String) -> Element {
    #[derive(serde::Deserialize, Debug)]
    struct DogApi {
        message: String,
    }

    let fut = use_future!(cx, |breed| async move {
        reqwest::get(format!("https://dog.ceo/api/breed/{breed}/images/random"))
            .await
            .unwrap()
            .json::<DogApi>()
            .await
    });

    match fut.value()? {
        Ok(resp) => render! {
            div {
                button {
                    onclick: move |_| {
                        println!("clicked");
                        fut.restart()
                    },
                    "Click to fetch another doggo"
                }
                img {
                    src: "{resp.message}",
                    max_width: "500px",
                    max_height: "500px",
                }
            }
        },
        Err(_) => render! { div { "loading dogs failed" } },
    }
}
