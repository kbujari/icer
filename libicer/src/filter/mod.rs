pub(crate) mod transform;

#[derive(Debug, Copy, Clone, PartialEq)]
#[rustfmt::skip]
pub enum FilterParams {
    A, B, C, D, E, F, Q,
}

impl FilterParams {
    #[rustfmt::skip]
    pub fn to_params(self) -> (f32, f32, f32, f32) {
        match self {
            Self::A => (0.0,        1.0/4.0,    1.0/4.0,    0.0),
            Self::B => (0.0,        2.0/8.0,    3.0/8.0,    2.0/8.0),
            Self::C => (-1.0/16.0,  4.0/16.0,   8.0/16.0,   6.0/16.0),
            Self::D => (0.0,        4.0/16.0,   5.0/16.0,   2.0/16.0),
            Self::E => (0.0,        3.0/16.0,   8.0/16.0,   6.0/16.0),
            Self::F => (0.0,        3.0/16.0,   9.0/16.0,   8.0/16.0),
            Self::Q => (0.0,        1.0/4.0,    1.0/4.0,    1.0/4.0),
        }
    }
}

impl std::str::FromStr for FilterParams {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "a" => Ok(FilterParams::A),
            "b" => Ok(FilterParams::B),
            "c" => Ok(FilterParams::C),
            "d" => Ok(FilterParams::D),
            "e" => Ok(FilterParams::E),
            "f" => Ok(FilterParams::F),
            "q" => Ok(FilterParams::Q),
            _ => Err("Possible filters: [A, B, C, D, E, F, Q]"),
        }
    }
}
