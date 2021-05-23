/** Describes a struct that can be used to fetch system resources **/
#[cfg(target_arch = "wasm32")]
pub struct SystemResources {}

#[cfg(not(target_arch = "wasm32"))]
pub struct SystemResources {
    info: sysinfo::System,
}

#[cfg(not(target_arch = "wasm32"))]
impl SystemResources {
    pub fn new() -> SystemResources {
        use sysinfo::{System, SystemExt};
        SystemResources {
            info: sysinfo::System::new_all(),
        }
    }

    pub fn poll(&mut self) {
        use sysinfo::SystemExt;
        self.info.refresh_all();
    }

    pub fn should_alloc(&self, bytes: u64) -> bool {
        use sysinfo::SystemExt;
        self.info.get_available_memory() > bytes
    }

    pub fn memory_warn(&self) {
        use sysinfo::SystemExt;
        log_warn!(format!(
            "Not enough memory to load chunks - {}/{}MB free ",
            self.info.get_available_memory() / 1000,
            self.info.get_total_memory() / 1000
        ));
    }
}

#[cfg(target_arch = "wasm32")]
impl SystemResources {
    pub fn new() -> SystemResources {
        SystemResources {}
    }

    pub fn poll(&mut self) {}

    pub fn should_alloc(&self, bytes: u64) -> bool {
        true
    }

    pub fn memory_warn(&self) {}
}
