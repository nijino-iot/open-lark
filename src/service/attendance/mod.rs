use crate::core::config::Config;

pub mod v1;

pub struct AttendanceService {
    pub v1: v1::V1,
}

impl AttendanceService {
    pub fn new(config: Config) -> Self {
        Self {
            v1: v1::V1::new(config),
        }
    }
}
