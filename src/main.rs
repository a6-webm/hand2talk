use std::{
    ffi::CString,
    iter::zip,
    sync::{Arc, Mutex},
};

use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    SampleFormat, SampleRate, Stream,
};
use hand2talk::{
    controller::ParamController,
    vtl::{VTLApi, VTLParams},
};

fn audio_setup(mut vtl_api: VTLApi, mx_params: Arc<Mutex<VTLParams>>) -> Stream {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("no output device available");
    let supported_configs_range = device
        .supported_output_configs()
        .expect("error while querying configs");
    let supported_config = {
        let mut sup_conf = None;
        for conf in supported_configs_range.into_iter() {
            if conf.channels() == 1 && conf.sample_format() == SampleFormat::F32 {
                sup_conf = Some(conf.with_sample_rate(SampleRate(44000)));
            }
        }
        sup_conf.expect("no supported config")
    };
    let err_fn = |err| eprintln!("an error occurred on the output audio stream: {}", err);
    let config = supported_config.into();
    device
        .build_output_stream(
            &config,
            move |data: &mut [f32], _| {
                let params = mx_params.lock().unwrap();
                let audio =
                    vtl_api.add_tract(data.len(), params.tract_state(), params.glottis_state());
                drop(params);
                for (d, s) in zip(data.iter_mut(), audio.iter().cloned()) {
                    *d = s as f32;
                }
            },
            err_fn,
            None,
        )
        .unwrap()
}

fn main() {
    let speaker = CString::new("./res/JD3.speaker").unwrap();
    let mut vtl_api = VTLApi::new(speaker).expect("failed to load speaker file");
    vtl_api.auto_calc_tr(true);
    let params = VTLParams::new();
    vtl_api.reset(params.tract_state(), params.glottis_state());
    let mx_data = Arc::new(Mutex::new(params));
    let stream = audio_setup(vtl_api, mx_data.clone());
    stream.play().unwrap();

    let mut controlling = ParamController::new(mx_data);
    controlling.controller_loop();
}
