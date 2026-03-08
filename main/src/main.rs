use dioxus::prelude::*;

use views::Home;

mod components;
mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Home {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/css/main.css");

fn main() {
    #[cfg(feature = "web")]
    console_error_panic_hook::set_once();

    dioxus::launch(App);
    /*
    #[cfg(not(feature = "server"))]
    dioxus::launch(App);

    #[cfg(feature = "server")]
    dioxus::serve(|| async move {
        //use dioxus::server::axum;
        use http::header::{HeaderName, HeaderValue};
        use tower::ServiceBuilder;
        use tower_http::set_header::SetResponseHeaderLayer;

        let middleware = ServiceBuilder::new()
            .layer(SetResponseHeaderLayer::if_not_present(
                HeaderName::from_static("cross-origin-opener-policy"),
                //HeaderName::from_static("Cross-Origin-Opener-Policy"),
                //HeaderName::from_static("cross-origin"),
                HeaderValue::from_static("same-origin"),
            ))
            .layer(SetResponseHeaderLayer::if_not_present(
                HeaderName::from_static("cross-origin-embedder-policy"),
                //HeaderName::from_static("Cross-Origin-Embedder-Policy"),
                //HeaderName::from_static("cross-origin-e"),
                HeaderValue::from_static("require-corp"),
            ));
        Ok(dioxus::server::router(App).layer(middleware))
    });
    */
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        Router::<Route> {}
    }
}
