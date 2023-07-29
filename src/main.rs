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

struct VTLParam {
    name: String,
    start_val: f64,
    min: f64,
    max: f64,
}

fn only_vals(vec: &Vec<VTLParam>) -> Vec<f64> {
    vec.iter().map(|p| p.start_val).collect()
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
    let tract_params = vec![
        VTLParam {
            name: "HX".to_owned(),
            min: 0.000,
            max: 1.000,
            start_val: 1.000,
        },
        VTLParam {
            name: "HY".to_owned(),
            min: -6.000,
            max: -3.500,
            start_val: -4.750,
        },
        VTLParam {
            name: "JX".to_owned(),
            min: -0.500,
            max: 0.000,
            start_val: 0.000,
        },
        VTLParam {
            name: "JA".to_owned(),
            min: -7.000,
            max: 0.000,
            start_val: -2.000,
        },
        VTLParam {
            name: "LP".to_owned(),
            min: -1.000,
            max: 1.000,
            start_val: -0.070,
        },
        VTLParam {
            name: "LD".to_owned(),
            min: -2.000,
            max: 4.000,
            start_val: 0.950,
        },
        VTLParam {
            name: "VS".to_owned(),
            min: 0.000,
            max: 1.000,
            start_val: 0.000,
        },
        VTLParam {
            name: "VO".to_owned(),
            min: -0.100,
            max: 1.000,
            start_val: -0.100,
        },
        VTLParam {
            name: "TCX".to_owned(),
            min: -3.000,
            max: 4.000,
            start_val: -0.400,
        },
        VTLParam {
            name: "TCY".to_owned(),
            min: -3.000,
            max: 1.000,
            start_val: -1.460,
        },
        VTLParam {
            name: "TTX".to_owned(),
            min: 1.500,
            max: 5.500,
            start_val: 3.500,
        },
        VTLParam {
            name: "TTY".to_owned(),
            min: -3.000,
            max: 2.500,
            start_val: -1.000,
        },
        VTLParam {
            name: "TBX".to_owned(),
            min: -3.000,
            max: 4.000,
            start_val: 2.000,
        },
        VTLParam {
            name: "TBY".to_owned(),
            min: -3.000,
            max: 5.000,
            start_val: 0.500,
        },
        VTLParam {
            name: "TRX".to_owned(),
            min: -4.000,
            max: 2.000,
            start_val: 0.000,
        },
        VTLParam {
            name: "TRY".to_owned(),
            min: -6.000,
            max: 0.000,
            start_val: 0.000,
        },
        VTLParam {
            name: "TS1".to_owned(),
            min: 0.000,
            max: 1.000,
            start_val: 0.000,
        },
        VTLParam {
            name: "TS2".to_owned(),
            min: 0.000,
            max: 1.000,
            start_val: 0.000,
        },
        VTLParam {
            name: "TS3".to_owned(),
            min: -1.000,
            max: 1.000,
            start_val: 0.000,
        },
    ];

    let glottis_params = vec![
        VTLParam {
            name: "F0".to_owned(),
            min: 40.000000,
            max: 600.000000,
            start_val: 120.000000,
        },
        VTLParam {
            name: "PR".to_owned(),
            min: 0.000000,
            max: 20000.000000,
            start_val: 0000.000000,
        },
        VTLParam {
            name: "XB".to_owned(),
            min: -0.050000,
            max: 0.300000,
            start_val: 0.010000,
        },
        VTLParam {
            name: "XT".to_owned(),
            min: -0.050000,
            max: 0.300000,
            start_val: 0.020000,
        },
        VTLParam {
            name: "CA".to_owned(),
            min: -0.250000,
            max: 0.250000,
            start_val: 0.050000,
        },
        VTLParam {
            name: "PL".to_owned(),
            min: 0.000000,
            max: 3.141500,
            start_val: 1.220000,
        },
        VTLParam {
            name: "RA".to_owned(),
            min: -1.000000,
            max: 1.000000,
            start_val: 1.000000,
        },
        VTLParam {
            name: "DP".to_owned(),
            min: 0.000000,
            max: 1.000000,
            start_val: 0.050000,
        },
        VTLParam {
            name: "PS".to_owned(),
            min: -0.500000,
            max: 0.500000,
            start_val: 0.000000,
        },
        VTLParam {
            name: "FL".to_owned(),
            min: 0.000000,
            max: 100.000000,
            start_val: 25.000000,
        },
        VTLParam {
            name: "AS".to_owned(),
            min: -40.000000,
            max: 0.000000,
            start_val: -10.000000,
        },
    ];

    let speaker = CString::new("./res/JD3.speaker").unwrap();
    let throat_vals: (Vec<f64>, Vec<f64>) = (
        tract_params.iter().map(|p| p.start_val).collect(),
        glottis_params.iter().map(|p| p.start_val).collect(),
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
            only_vals(&tract_params).as_mut_ptr(),
            only_vals(&glottis_params).as_mut_ptr(),
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
