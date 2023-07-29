use std::{
    ffi::CString,
    iter::zip,
    ptr::null_mut,
    sync::{Arc, Mutex},
    thread::sleep,
    time::Duration,
};

use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    SampleFormat, SampleRate, Stream,
};
use hand2talk::{GLOTTIS_PARAMS, TRACT_PARAMS};

extern "C" {
    fn vtlInitialize(speakerFileName: *const i8) -> i32;
    fn vtlClose() -> i32;
    fn vtlCalcTongueRootAutomatically(automaticCalculation: bool) -> i32;
    fn vtlSynthesisReset() -> i32;
    fn vtlSynthesisAddTract(
        numNewSamples: i32,
        audio: *mut f64,
        tractParams: *mut f64,
        glottisParams: *mut f64,
    ) -> i32;
}

fn audio_setup(mx_data: Arc<Mutex<(Vec<f64>, Vec<f64>)>>) -> Stream {
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
            move |data: &mut [f32], _| unsafe {
                let mut audio: Vec<f64> = vec![0.0; data.len()];
                let mut params = mx_data.lock().unwrap();
                vtlSynthesisAddTract(
                    data.len() as i32,
                    audio.as_mut_ptr(),
                    params.0.as_mut_ptr(),
                    params.1.as_mut_ptr(),
                );
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
    let mut throat_vals: (Vec<f64>, Vec<f64>) = (
        TRACT_PARAMS.iter().map(|p| p.start_val).collect(),
        GLOTTIS_PARAMS.iter().map(|p| p.start_val).collect(),
    );
    unsafe {
        if vtlInitialize(speaker.as_ptr()) == 1 {
            panic!("failed to load speaker file")
        };
        vtlCalcTongueRootAutomatically(true);
        vtlSynthesisReset();
        vtlSynthesisAddTract(
            0,
            null_mut(),
            throat_vals.0.as_mut_ptr(),
            throat_vals.1.as_mut_ptr(),
        );
    }
    let mx_data = Arc::new(Mutex::new(throat_vals));
    let stream = audio_setup(mx_data.clone());
    stream.play().unwrap();

    for i in 0..100 {
        let mut vals = mx_data.lock().unwrap();
        println!("{i}");
        vals.1[1] += 200.0;
        drop(vals);
        sleep(Duration::from_millis(20));
    }

    drop(stream);
    unsafe {
        vtlClose();
    }
}
