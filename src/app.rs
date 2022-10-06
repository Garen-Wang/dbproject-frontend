use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::{AppRoute, switch};
use crate::components::header::Header;
use crate::components::user_context_provider::UserContextProvider;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <UserContextProvider>
        <BrowserRouter>
            <Header />
            <Switch<AppRoute> render={Switch::render(switch)} />
        </BrowserRouter>
        </UserContextProvider>
    }
}