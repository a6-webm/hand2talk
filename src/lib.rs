pub struct VTLParam<'a> {
    pub name: &'a str,
    pub start_val: f64,
    pub min: f64,
    pub max: f64,
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
