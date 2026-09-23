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
unsafe extern "C" {
    fn gpu_x_get_telemetry(
        gpu_type: i32,
        out: *mut GpuTelemetry,
    ) -> bool;
}

#[cfg(windows)]
pub fn get_telemetry(gpu_type: i32) -> Option<GpuTelemetry> {
    let mut telemetry = GpuTelemetry::default();

    let success = unsafe {
        gpu_x_get_telemetry(
            gpu_type,
            &mut telemetry,
        )
    };

    if success {
        Some(telemetry)
    } else {
        None
    }
}

#[cfg(not(windows))]
pub fn get_telemetry(_gpu_type: i32) -> Option<GpuTelemetry> {
    None
}