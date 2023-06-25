use leptos::{ev::MouseEvent, *};
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="Tim Ruland"/>

        // content for this welcome page
        <Router>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                </Routes>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    view! { cx,
        //<!-- Start Landing Page https://codepen.io/Mohamed-Anwar97/pen/yLOwGed -->
        <div class="landing-page">
            <Header/>
            <main class="content">
                <div class="container">
                    <div class="info">
                        <h1>"Looking For Inspiration"</h1>
                        <p>"Lorem ipsum dolor sit amet consectetur adipisicing elit. Repellendus odit nihil ullam nesciunt quidem iste, Repellendus odit nihil"</p>
                        <button>"Button name"</button>
                    </div>
                    <div class="image">
                        <img src="https://i.postimg.cc/65QxYYzh/001234.png"/>
                    </div>
                </div>
            </main>
        </div>
        //<!-- End Landing Page -->
    }
}
#[component]
pub fn Header(cx: Scope) -> impl IntoView {
    view! { cx,
        <header>
            <div class="container">
                <a href="#" class="logo">"Your" <b>"Website"</b></a>
                <ul class="links">
                    <li><a href="#">"Home"</a></li>
                    <li><a href="#">"About Us"</a></li>
                    <li><a href="#">"Work"</a></li>
                    <li><a href="#">"Info"</a></li>
                    <li><a href="#">"Get Started"</a></li>
                </ul>
            </div>
        </header>
    }
}
