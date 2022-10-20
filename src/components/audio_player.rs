use web_sys::{HtmlAudioElement, HtmlInputElement};
use yew::prelude::*;
use yew_hooks::use_media;
use yew_icons::{Icon, IconId};

#[function_component(AudioPlayer)]
pub fn audio_player() -> Html {
    let is_playing = use_state(|| false);

    let audio_ref = use_node_ref();
    let progress_bar_ref = use_node_ref();
    let volume_bar_ref = use_node_ref();

    let audio = use_media(
        audio_ref.clone(),
        "http://commondatastorage.googleapis.com/codeskulptor-demos/DDR_assets/Kangaroo_MusiQue_-_The_Neverwritten_Role_Playing_Game.mp3".to_string()
    );

    let onclick_play_pause = {
        let is_playing = is_playing.clone();
        let audio_ref = audio_ref.clone();
        Callback::from(move |_: MouseEvent| {
            let prev_state = *is_playing;
            is_playing.set(!prev_state);
            let audio = audio_ref.cast::<HtmlAudioElement>().unwrap();
            if prev_state {
                log::debug!("pause");
                audio.pause().unwrap();
            } else {
                log::debug!("play");
                audio.play().unwrap();
            }
        })
    };

    let onchange_progress_bar = {
        let progress_bar_ref = progress_bar_ref.clone();
        let audio = audio.clone();
        Callback::from(move |_| {
            let progress_bar = progress_bar_ref.cast::<HtmlInputElement>().unwrap();
            let cur_time: f64 = progress_bar.value().parse().unwrap();
            audio.seek(cur_time);
        })
    };

    let onchange_volume_bar = {
        let volume_bar_ref = volume_bar_ref.clone();
        let audio = audio.clone();
        Callback::from(move |_| {
            let volume_bar = volume_bar_ref.cast::<HtmlInputElement>().unwrap();
            let cur_vol = volume_bar.value().parse::<f64>().unwrap() * 0.01;
            audio.set_volume(cur_vol);
        })
    };

    let backward_ten_secs = {
        let audio = audio.clone();
        let progress_bar_ref = progress_bar_ref.clone();
        Callback::from(move |_| {
            let prev_time = *audio.time as i32;
            let cur_time = (prev_time - 10).max(0);
            audio.seek(cur_time as f64);
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
            audio.seek(cur_time as f64);
            let progress_bar = progress_bar_ref.cast::<HtmlInputElement>().unwrap();
            progress_bar.set_value(&cur_time.to_string());
        })
    };

    html! {
        <div>
            <audio ref={audio_ref} preload="metadata"></audio>
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
                <input ref={progress_bar_ref} type="range" max={((*audio.duration) as i32).to_string()} value={((*audio.time) as i32).to_string()} onchange={onchange_progress_bar} />
            </div>

            <div>{formatted_duration(*audio.duration)}</div>

            <div>
                <input ref={volume_bar_ref} type="range" max="100" value={((*audio.volume * 100.0) as i32).to_string()} onchange={onchange_volume_bar} />
            </div>
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
