use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
#[repr(C)]
pub struct UpdateLoading {
    pub loading: bool,
}

impl UpdateLoading {
    pub fn new(loading: bool) -> UpdateLoading {
        UpdateLoading { loading }
    }
}
