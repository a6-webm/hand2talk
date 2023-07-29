use std::{ffi::CString, time::Instant};

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
    val: f64,
    min: f64,
    max: f64,
}

fn main() {
    let tract_params = vec![
        VTLParam {
            name: "HX".to_owned(),
            min: 0.000,
            max: 1.000,
            val: 1.000,
        },
        VTLParam {
            name: "HY".to_owned(),
            min: -6.000,
            max: -3.500,
            val: -4.750,
        },
        VTLParam {
            name: "JX".to_owned(),
            min: -0.500,
            max: 0.000,
            val: 0.000,
        },
        VTLParam {
            name: "JA".to_owned(),
            min: -7.000,
            max: 0.000,
            val: -2.000,
        },
        VTLParam {
            name: "LP".to_owned(),
            min: -1.000,
            max: 1.000,
            val: -0.070,
        },
        VTLParam {
            name: "LD".to_owned(),
            min: -2.000,
            max: 4.000,
            val: 0.950,
        },
        VTLParam {
            name: "VS".to_owned(),
            min: 0.000,
            max: 1.000,
            val: 0.000,
        },
        VTLParam {
            name: "VO".to_owned(),
            min: -0.100,
            max: 1.000,
            val: -0.100,
        },
        VTLParam {
            name: "TCX".to_owned(),
            min: -3.000,
            max: 4.000,
            val: -0.400,
        },
        VTLParam {
            name: "TCY".to_owned(),
            min: -3.000,
            max: 1.000,
            val: -1.460,
        },
        VTLParam {
            name: "TTX".to_owned(),
            min: 1.500,
            max: 5.500,
            val: 3.500,
        },
        VTLParam {
            name: "TTY".to_owned(),
            min: -3.000,
            max: 2.500,
            val: -1.000,
        },
        VTLParam {
            name: "TBX".to_owned(),
            min: -3.000,
            max: 4.000,
            val: 2.000,
        },
        VTLParam {
            name: "TBY".to_owned(),
            min: -3.000,
            max: 5.000,
            val: 0.500,
        },
        VTLParam {
            name: "TRX".to_owned(),
            min: -4.000,
            max: 2.000,
            val: 0.000,
        },
        VTLParam {
            name: "TRY".to_owned(),
            min: -6.000,
            max: 0.000,
            val: 0.000,
        },
        VTLParam {
            name: "TS1".to_owned(),
            min: 0.000,
            max: 1.000,
            val: 0.000,
        },
        VTLParam {
            name: "TS2".to_owned(),
            min: 0.000,
            max: 1.000,
            val: 0.000,
        },
        VTLParam {
            name: "TS3".to_owned(),
            min: -1.000,
            max: 1.000,
            val: 0.000,
        },
    ];

    let glottis_params = vec![
        VTLParam {
            name: "F0".to_owned(),
            min: 40.000000,
            max: 600.000000,
            val: 120.000000,
        },
        VTLParam {
            name: "PR".to_owned(),
            min: 0.000000,
            max: 20000.000000,
            val: 8000.000000,
        },
        VTLParam {
            name: "XB".to_owned(),
            min: -0.050000,
            max: 0.300000,
            val: 0.010000,
        },
        VTLParam {
            name: "XT".to_owned(),
            min: -0.050000,
            max: 0.300000,
            val: 0.020000,
        },
        VTLParam {
            name: "CA".to_owned(),
            min: -0.250000,
            max: 0.250000,
            val: 0.050000,
        },
        VTLParam {
            name: "PL".to_owned(),
            min: 0.000000,
            max: 3.141500,
            val: 1.220000,
        },
        VTLParam {
            name: "RA".to_owned(),
            min: -1.000000,
            max: 1.000000,
            val: 1.000000,
        },
        VTLParam {
            name: "DP".to_owned(),
            min: 0.000000,
            max: 1.000000,
            val: 0.050000,
        },
        VTLParam {
            name: "PS".to_owned(),
            min: -0.500000,
            max: 0.500000,
            val: 0.000000,
        },
        VTLParam {
            name: "FL".to_owned(),
            min: 0.000000,
            max: 100.000000,
            val: 25.000000,
        },
        VTLParam {
            name: "AS".to_owned(),
            min: -40.000000,
            max: 0.000000,
            val: -10.000000,
        },
    ];

    unsafe fn to_raw_vals(vec: &Vec<VTLParam>) -> *mut f64 {
        let mut out: Vec<f64> = vec.iter().map(|p| p.val).collect();
        return out.as_mut_ptr();
    }

    let speaker = CString::new("./res/JD3.speaker").unwrap();
    let num_samples: usize = 220;
    let mut audio: Vec<f64> = vec![0.0; num_samples];
    unsafe {
        if vtlInitialize(speaker.as_ptr()) == 1 {
            panic!("failed to load speaker file")
        };
        vtlCalcTongueRootAutomatically(true);
        vtlSynthesisReset();
        vtlSynthesisAddTract(
            0,
            audio.as_mut_ptr(),
            to_raw_vals(&tract_params),
            to_raw_vals(&glottis_params),
        );
        let timer = Instant::now();
        vtlSynthesisAddTract(
            num_samples as i32,
            audio.as_mut_ptr(),
            to_raw_vals(&tract_params),
            to_raw_vals(&glottis_params),
        );
        let elapsed = timer.elapsed().as_secs_f64();
        println!("Took {elapsed} seconds");
        vtlClose();
    }
}
