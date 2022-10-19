use yew::prelude::*;
use crate::components::audio_player::AudioPlayer;
#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <AudioPlayer />
    }
}