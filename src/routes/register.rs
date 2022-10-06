use yew::prelude::*;

#[function_component(Register)]
pub fn register() -> Html {
    html! {
        <div class="auth-page">
        <div class="container page">
        <div class="row">
        <div class="col-md-6 offset-md-3 col-xs-12">
        <h1 class="text-xs-center"> {"Register"} </h1>
        <form>
        <fieldset>
        <fieldset class="form-group">
        <input type="text" class="form-control form-control-lg" />
        </fieldset>
        <fieldset class="form-group">
        <input type="email" class="form-control form-control-lg" />
        </fieldset>
        <fieldset class="form-group">
        <input type="password" class="form-control form-control-lg" />
        </fieldset>
        <button type="submit"> {"Register"} </button>
        </fieldset>
        </form>
        </div>
        </div>
        </div>
        </div>
    }
}