use battleship_solver::{self, structs::BoardBuilder};
use leptos::*;
use leptos_meta::*;

#[component]
pub fn Battleship(cx: Scope) -> impl IntoView {
    let (board_builder, set_board_builder) = create_signal(cx, BoardBuilder::default());
    view! { cx,
        <Title text="Battleship Solver"/>
        <h1>"Battleship Solver"</h1>
        <main>"Battleship Test"</main>
    }
}
