#[derive(Debug, Clone, Copy)]
pub enum Filter {
    HighPass,
    LowPasks,
}

impl Filter {
    pub fn transform(self, data: Vec<u8>, params: super::FilterParams) -> Vec<u8> {
        match self {
            Self::HighPass => Self::high_pass_transform(data, params),
            Self::LowPasks => Self::low_pass_transform(data),
        }
    }

    fn high_pass_transform(data: Vec<u8>, params: super::FilterParams) -> Vec<u8> {
        data
    }

    fn low_pass_transform(data: Vec<u8>) -> Vec<u8> {
        data
    }
}
