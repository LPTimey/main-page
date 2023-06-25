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
                <article class="container">
                    <div class="info">
                        <h1>"Looking For Inspiration"</h1>
                        <p>"Lorem ipsum dolor sit amet consectetur adipisicing elit. Repellendus odit nihil ullam nesciunt quidem iste, Repellendus odit nihil"</p>
                        <button>"Button name"</button>
                    </div>
                    <div class="image">
                        <img src="https://i.postimg.cc/65QxYYzh/001234.png"/>
                    </div>
                </article>
            </main>
            <Footer/>
        </div>
        //<!-- End Landing Page -->
    }
}
#[component]
pub fn Header(cx: Scope) -> impl IntoView {
    view! { cx,
        <header>
            <div class="container">
                <a href="#" class="logo">"Tim" <b>"Ruland"</b></a>
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

#[component]
pub fn Footer(cx: Scope) -> impl IntoView {
    view! {cx,
        <footer>
            <address class="container">
                <a class="img_link" href="https://github.com/LPTimey" target="_blank" rel="noopener noreferrer">
                    <svg height="32" aria-hidden="true" viewBox="0 0 16 16" version="1.1" width="32" data-view-component="true" class="octicon octicon-mark-github v-align-middle color-fg-default">
                        <path d="M8 0c4.42 0 8 3.58 8 8a8.013 8.013 0 0 1-5.45 7.59c-.4.08-.55-.17-.55-.38 0-.27.01-1.13.01-2.2 0-.75-.25-1.23-.54-1.48 1.78-.2 3.65-.88 3.65-3.95 0-.88-.31-1.59-.82-2.15.08-.2.36-1.02-.08-2.12 0 0-.67-.22-2.2.82-.64-.18-1.32-.27-2-.27-.68 0-1.36.09-2 .27-1.53-1.03-2.2-.82-2.2-.82-.44 1.1-.16 1.92-.08 2.12-.51.56-.82 1.28-.82 2.15 0 3.06 1.86 3.75 3.64 3.95-.23.2-.44.55-.51 1.07-.46.21-1.61.55-2.33-.66-.15-.24-.6-.83-1.23-.82-.67.01-.27.38.01.53.34.19.73.9.82 1.13.16.45.68 1.31 2.69.94 0 .67.01 1.3.01 1.49 0 .21-.15.45-.55.38A7.995 7.995 0 0 1 0 8c0-4.42 3.58-8 8-8Z"></path>
                    </svg>
                    "My Github"
                </a>
            </address>
        </footer>
    }
}
