use std::str::FromStr;

use avina::{Api, Token};
use dioxus::prelude::*;
use typst_pdf::PdfOptions;

mod typst_wrapper;
use typst_wrapper::TypstWrapperWorld;

fn main() {
    launch(app);
}

fn app() -> Element {
    let future = use_resource(move || async move {
        let mut eval = document::eval(
            r#"
            window.addEventListener("message", function(event) {
                let token = event.data;
                dioxus.send(token);
            });
            window.parent.postMessage("request-token", "*");
            "#,
        );
        let token_str: String = eval.recv().await.unwrap();
        let token = Token::from_str(&token_str).unwrap();
        let api = Api::new(
            "http://localhost:8000/api".to_string(),
            token,
            None,
            None,
        )
        .unwrap();
        api.user.me().await.unwrap()
    });
    match future.read_unchecked().as_ref() {
        Some(user) => {
            rsx! {
                p { "Hello {user.name} from Dioxus!" },
                button {
                    onclick: |_| {
                        let content = "= Hello, World!";
                        let world = TypstWrapperWorld::new("./examples".to_owned(), content.to_owned());
                        let document = typst::compile(&world)
                            .output
                            .expect("Error compiling typst");
                        let pdf = typst_pdf::pdf(&document, &PdfOptions::default()).expect("Error exporting PDF");
                        std::fs::write("./output.pdf", pdf).expect("Error writing PDF.");
                    },
                    "do something"
                }
            }
        }
        _ => rsx! { p { "No token provided!" } },
    }
}
