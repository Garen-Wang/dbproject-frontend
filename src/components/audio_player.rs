use web_sys::{HtmlAudioElement, HtmlInputElement};
use yew::prelude::*;
use yew_hooks::use_media;
use yew_icons::{Icon, IconId};

#[function_component(AudioPlayer)]
pub fn audio_player() -> Html {
    let is_playing = use_state(|| false);

    let audio_ref = use_node_ref();
    let progress_bar_ref = use_node_ref();

    let audio = use_media(audio_ref.clone(), "http://commondatastorage.googleapis.com/codeskulptor-demos/DDR_assets/Kangaroo_MusiQue_-_The_Neverwritten_Role_Playing_Game.mp3".to_string());

    let onclick_play_pause = {
        let is_playing = is_playing.clone();
        let audio_ref = audio_ref.clone();
        Callback::from(move |_: MouseEvent| {
            let prev_state = *is_playing;
            is_playing.set(!prev_state);
            let audio = audio_ref.cast::<HtmlAudioElement>().unwrap();
            if prev_state {
                log::debug!("pause");
                audio.pause().unwrap(); // how to handle promise?
            } else {
                log::debug!("play");
                let _promise = audio.play().unwrap(); // how to handle promise?
            }
        })
    };

    let onchange = {
        let progress_bar_ref = progress_bar_ref.clone();
        let audio = audio.clone();
        Callback::from(move |_| {
            let progress_bar = progress_bar_ref.cast::<HtmlInputElement>().unwrap();
            let cur_time: f64 = progress_bar.value().parse().unwrap();
            audio.time.set(cur_time);
        })
    };

    let backward_ten_secs = {
        let audio = audio.clone();
        let progress_bar_ref = progress_bar_ref.clone();
        Callback::from(move |_| {
            let prev_time = *audio.time as i32;
            let cur_time = (prev_time - 10).max(0);
            audio.time.set(cur_time as f64);

            let progress_bar = progress_bar_ref.cast::<HtmlInputElement>().unwrap();
            progress_bar.set_value(&cur_time.to_string());
        })
    };

    let forward_ten_secs = {
        let audio = audio.clone();
        let progress_bar_ref = progress_bar_ref.clone();
        Callback::from(move |_| {
            let prev_time = *audio.time as i32;
            let duration = *audio.duration as i32;
            let cur_time = (prev_time + 10).min(duration);
            audio.time.set(cur_time as f64);

            let progress_bar = progress_bar_ref.cast::<HtmlInputElement>().unwrap();
            progress_bar.set_value(&cur_time.to_string());
        })
    };

    html! {
        <div>
            <audio ref={audio_ref}></audio>
            <button onclick={backward_ten_secs}>{"10s backward"}</button>
            <button onclick={onclick_play_pause}>
            {
                if *is_playing {
                    pause_button()
                } else {
                    play_button()
                }
            }
            </button>
            <button onclick={forward_ten_secs}>{"10s forward"}</button>

            <div>{formatted_duration(*audio.time)}</div>

            <div>
                <input ref={progress_bar_ref} type="range" max={((*audio.duration) as i32).to_string()} onchange={onchange} />
            </div>

            <div>{formatted_duration(*audio.duration)}</div>
        </div>
    }
}

fn formatted_duration(duration: f64) -> String {
    let duration = duration as i32;
    let mins = duration / 60;
    let secs = duration % 60;
    format!("{:0>2}:{:0>2}", mins, secs)
}

fn play_button() -> Html {
    html! {
        <Icon icon_id={IconId::LucidePlay} />
    }
}

fn pause_button() -> Html {
    html! {
        <Icon icon_id={IconId::LucidePause} />
    }
}
