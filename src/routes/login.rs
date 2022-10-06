use yew::prelude::*;

#[function_component(Login)]
pub fn login() -> Html {
    html! {
        <div class="auth-page">
        <div class="container page">
        <div class="row">
        <div class="col-md-6 offset-md-3 col-xs-12">
        <h1 class="text-xs-center"> {"Login"} </h1>
        <form>
        <fieldset>
        <fieldset class="form-group"></fieldset>
        <input type="email" name="email" id="email" class="form-control form-control-lg" />
        <fieldset class="form-group"></fieldset>
        <input type="password" name="password" id="password" class="form-control form-control-lg" />
        <button type="submit"> {"Login"} </button>
        </fieldset>
        </form>
        </div>
        </div>
        </div>
        </div>
    }
}
