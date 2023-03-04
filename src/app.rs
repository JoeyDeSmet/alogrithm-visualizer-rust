use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::pages::home::*;

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
    <Title text="Algorithm visualizer"/>

    // content for this welcome page
    <Router>
      <main>
        <Routes>
        
          <Route path="" view=|cx| view! { cx, 
            <HomePage/> 
          }/>
        
        </Routes>
      </main>
    </Router>
  }
}


