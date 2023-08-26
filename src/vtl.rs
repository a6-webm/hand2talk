use std::{
    ffi::{c_char, CString},
    ptr::null_mut,
};

extern "C" {
    fn vtlInitialize(speakerFileName: *const c_char) -> i32;
    fn vtlClose() -> i32;
    fn vtlCalcTongueRootAutomatically(automaticCalculation: bool) -> i32;
    fn vtlSynthesisReset() -> i32;
    fn vtlSynthesisAddTract(
        numNewSamples: i32,
        audio: *mut f64,
        tractParams: *const f64,
        glottisParams: *const f64,
    ) -> i32;
}

pub struct VTLParam<'a> {
    pub name: &'a str,
    pub start_val: f64,
    pub min: f64,
    pub max: f64,
}

#[non_exhaustive]
pub struct VTLApi();

impl VTLApi {
    pub fn new(speaker: CString) -> Option<VTLApi> {
        match unsafe { vtlInitialize(speaker.as_ptr()) } {
            1 => None,
            _ => Some(VTLApi()),
        }
    }

    pub fn auto_calc_tr(&mut self, auto_calc: bool) {
        unsafe {
            vtlCalcTongueRootAutomatically(auto_calc);
        }
    }

    pub fn reset(&mut self, init_tract: &[f64], init_glottis: &[f64]) {
        unsafe {
            vtlSynthesisReset();
            vtlSynthesisAddTract(0, null_mut(), init_tract.as_ptr(), init_glottis.as_ptr());
        }
    }

    pub fn add_tract(
        &mut self,
        num_samples: usize,
        tract_params: &[f64],
        glottis_params: &[f64],
    ) -> Vec<f64> {
        let mut audio: Vec<f64> = vec![0.0; num_samples];
        unsafe {
            vtlSynthesisAddTract(
                num_samples as i32,
                audio.as_mut_ptr(),
                tract_params.as_ptr(),
                glottis_params.as_ptr(),
            );
        }
        audio
    }
}

impl Drop for VTLApi {
    fn drop(&mut self) {
        unsafe {
            vtlClose();
        }
    }
}

pub struct VTLParams {
    throat_state: Vec<f64>,
    glottis_state: Vec<f64>,
}

impl VTLParams {
    pub fn new() -> VTLParams {
        VTLParams {
            throat_state: TRACT_PARAMS.iter().map(|p| p.start_val).collect(),
            glottis_state: GLOTTIS_PARAMS.iter().map(|p| p.start_val).collect(),
        }
    }

    pub fn set_throat_value(&mut self, idx: TractIdx, normalised: f64) {
        let param = &TRACT_PARAMS[idx as usize];
        let val = param.min + (param.max - param.min) * normalised;
        self.throat_state[idx as usize] = val;
    }

    pub fn set_glottis_value(&mut self, idx: GlottisIdx, normalised: f64) {
        let param = &GLOTTIS_PARAMS[idx as usize];
        let val = param.min + (param.max - param.min) * normalised;
        self.glottis_state[idx as usize] = val;
    }

    pub fn throat_state(&self) -> &[f64] {
        &self.throat_state
    }

    pub fn glottis_state(&self) -> &[f64] {
        &self.glottis_state
    }
}

#[derive(Clone, Copy)]
pub enum TractIdx {
    HX = 0,
    HY,
    JX,
    JA,
    LP,
    LD,
    VS,
    VO,
    TCX,
    TCY,
    TTX,
    TTY,
    TBX,
    TBY,
    TRX,
    TRY,
    TS1,
    TS2,
    TS3,
}

#[derive(Clone, Copy)]
pub enum GlottisIdx {
    F0 = 0,
    PR,
    XB,
    XT,
    CA,
    PL,
    RA,
    DP,
    PS,
    FL,
    AS,
}

pub const TRACT_PARAMS: &[VTLParam] = &[
    VTLParam {
        name: "HX",
        min: 0.000,
        max: 1.000,
        start_val: 1.000,
    },
    VTLParam {
        name: "HY",
        min: -6.000,
        max: -3.500,
        start_val: -4.750,
    },
    VTLParam {
        name: "JX",
        min: -0.500,
        max: 0.000,
        start_val: 0.000,
    },
    VTLParam {
        name: "JA",
        min: -7.000,
        max: 0.000,
        start_val: -2.000,
    },
    VTLParam {
        name: "LP",
        min: -1.000,
        max: 1.000,
        start_val: -0.070,
    },
    VTLParam {
        name: "LD",
        min: -2.000,
        max: 4.000,
        start_val: 0.950,
    },
    VTLParam {
        name: "VS",
        min: 0.000,
        max: 1.000,
        start_val: 0.000,
    },
    VTLParam {
        name: "VO",
        min: -0.100,
        max: 1.000,
        start_val: -0.100,
    },
    VTLParam {
        name: "TCX",
        min: -3.000,
        max: 4.000,
        start_val: -0.400,
    },
    VTLParam {
        name: "TCY",
        min: -3.000,
        max: 1.000,
        start_val: -1.460,
    },
    VTLParam {
        name: "TTX",
        min: 1.500,
        max: 5.500,
        start_val: 3.500,
    },
    VTLParam {
        name: "TTY",
        min: -3.000,
        max: 2.500,
        start_val: -1.000,
    },
    VTLParam {
        name: "TBX",
        min: -3.000,
        max: 4.000,
        start_val: 2.000,
    },
    VTLParam {
        name: "TBY",
        min: -3.000,
        max: 5.000,
        start_val: 0.500,
    },
    VTLParam {
        name: "TRX",
        min: -4.000,
        max: 2.000,
        start_val: 0.000,
    },
    VTLParam {
        name: "TRY",
        min: -6.000,
        max: 0.000,
        start_val: 0.000,
    },
    VTLParam {
        name: "TS1",
        min: 0.000,
        max: 1.000,
        start_val: 0.000,
    },
    VTLParam {
        name: "TS2",
        min: 0.000,
        max: 1.000,
        start_val: 0.000,
    },
    VTLParam {
        name: "TS3",
        min: -1.000,
        max: 1.000,
        start_val: 0.000,
    },
];

pub const GLOTTIS_PARAMS: &[VTLParam] = &[
    VTLParam {
        name: "F0",
        min: 40.000000,
        max: 600.000000,
        start_val: 120.000000,
    },
    VTLParam {
        name: "PR",
        min: 0.000000,
        max: 20000.000000,
        start_val: 0000.000000,
    },
    VTLParam {
        name: "XB",
        min: -0.050000,
        max: 0.300000,
        start_val: 0.010000,
    },
    VTLParam {
        name: "XT",
        min: -0.050000,
        max: 0.300000,
        start_val: 0.020000,
    },
    VTLParam {
        name: "CA",
        min: -0.250000,
        max: 0.250000,
        start_val: 0.050000,
    },
    VTLParam {
        name: "PL",
        min: 0.000000,
        max: 3.141500,
        start_val: 1.220000,
    },
    VTLParam {
        name: "RA",
        min: -1.000000,
        max: 1.000000,
        start_val: 1.000000,
    },
    VTLParam {
        name: "DP",
        min: 0.000000,
        max: 1.000000,
        start_val: 0.050000,
    },
    VTLParam {
        name: "PS",
        min: -0.500000,
        max: 0.500000,
        start_val: 0.000000,
    },
    VTLParam {
        name: "FL",
        min: 0.000000,
        max: 100.000000,
        start_val: 25.000000,
    },
    VTLParam {
        name: "AS",
        min: -40.000000,
        max: 0.000000,
        start_val: -10.000000,
    },
];
