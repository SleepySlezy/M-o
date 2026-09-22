#[cfg(windows)]
#[allow(dead_code)]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct GpuTelemetry {
    pub gpu_usage: f64,
    pub gpu_clock: f64,
    pub memory_clock: f64,
    pub temperature: f64,
    pub power: f64,
    pub fan_rpm: f64,
}

#[cfg(windows)]
#[allow(dead_code)]
unsafe extern "C" {
    fn gpu_x_get_telemetry(out: *mut GpuTelemetry) -> bool;
}

#[cfg(windows)]
#[allow(dead_code)]
pub fn get_telemetry() -> Option<GpuTelemetry> {
    println!("ADLX: BEFORE FFI");

    let mut telemetry = GpuTelemetry::default();

    let success = unsafe {
        gpu_x_get_telemetry(&mut telemetry)
    };

    println!("ADLX: AFTER FFI");

    if success {
        Some(telemetry)
    } else {
        None
    }
}

#[cfg(not(windows))]
#[allow(dead_code)]
pub fn get_telemetry() -> Option<GpuTelemetry> {
    None
}