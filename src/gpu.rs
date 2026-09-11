#[cfg(windows)]
use windows::{
    core::Interface,
    Win32::Graphics::Dxgi::{
        CreateDXGIFactory1,
        IDXGIAdapter1,
        IDXGIAdapter3,
        IDXGIFactory6,
        DXGI_ADAPTER_FLAG_SOFTWARE,
        DXGI_GPU_PREFERENCE_HIGH_PERFORMANCE,
        DXGI_MEMORY_SEGMENT_GROUP_LOCAL,
    },
};

#[derive(Clone, Debug)]
pub struct GpuInfo {
    pub supported: bool,

    // GPU
    pub name: String,
    pub architecture: String,
    pub vendor: String,
    pub vendor_id: String,
    pub device_id: String,
    pub pci_location: String,
    pub gpu_type: String,

    // Memory
    pub vram_total: String,
    pub vram_used: String,
    pub vram_available: String,
    pub memory_type: String,
    pub memory_bus: String,
    pub memory_bandwidth: String,

    // Clocks
    pub gpu_clock: String,
    pub memory_clock: String,
    pub boost_clock: String,

    // Sensors
    pub gpu_usage: String,
    pub temperature: String,
    pub power: String,
    pub fan: String,

    // Software
    pub driver: String,
    pub driver_version: String,
    pub vbios: String,
}

impl GpuInfo {
    pub fn empty() -> Self {
        Self {
            supported: false,

            name: "N/A".into(),
            architecture: "N/A".into(),
            vendor: "N/A".into(),
            vendor_id: "N/A".into(),
            device_id: "N/A".into(),
            pci_location: "N/A".into(),
            gpu_type: "N/A".into(),

            vram_total: "N/A".into(),
            vram_used: "N/A".into(),
            vram_available: "N/A".into(),
            memory_type: "N/A".into(),
            memory_bus: "N/A".into(),
            memory_bandwidth: "N/A".into(),

            gpu_clock: "N/A".into(),
            memory_clock: "N/A".into(),
            boost_clock: "N/A".into(),

            gpu_usage: "N/A".into(),
            temperature: "N/A".into(),
            power: "N/A".into(),
            fan: "N/A".into(),

            driver: "N/A".into(),
            driver_version: "N/A".into(),
            vbios: "N/A".into(),
        }
    }

    #[cfg(windows)]
    pub fn detect() -> Self {
        unsafe {
            match detect_windows() {
                Ok(info) => info,
                Err(_) => Self::empty(),
            }
        }
    }

    #[cfg(not(windows))]
    pub fn detect() -> Self {
        Self::empty()
    }

    pub fn as_text(&self) -> String {
        format!(
r#"GPU-X 0.1
==============================

GPU
Name: {}
Architecture: {}
Vendor: {}
PCI Vendor ID: {}
PCI Device ID: {}
PCI Location: {}
GPU Type: {}

Memory
VRAM: {}
VRAM Used: {}
VRAM Available: {}
Memory Type: {}
Memory Bus: {}
Memory Bandwidth: {}

Clocks
GPU Clock: {}
Memory Clock: {}
Boost Clock: {}

Sensors
GPU Usage: {}
Temperature: {}
Power: {}
Fan: {}

Software
Driver: {}
Driver Version: {}
VBIOS: {}
"#,
            self.name,
            self.architecture,
            self.vendor,
            self.vendor_id,
            self.device_id,
            self.pci_location,
            self.gpu_type,

            self.vram_total,
            self.vram_used,
            self.vram_available,
            self.memory_type,
            self.memory_bus,
            self.memory_bandwidth,

            self.gpu_clock,
            self.memory_clock,
            self.boost_clock,

            self.gpu_usage,
            self.temperature,
            self.power,
            self.fan,

            self.driver,
            self.driver_version,
            self.vbios,
        )
    }
}

#[cfg(windows)]
unsafe fn detect_windows() -> windows::core::Result<GpuInfo> {
    let factory: IDXGIFactory6 = unsafe { CreateDXGIFactory1()? };

    let mut adapters: Vec<IDXGIAdapter1> = Vec::new();

    let mut index = 0;

    loop {
        match unsafe {
            factory.EnumAdapterByGpuPreference(
                index,
                DXGI_GPU_PREFERENCE_HIGH_PERFORMANCE,
            )
        } {
            Ok(adapter) => {
                adapters.push(adapter);
                index += 1;
            }

            Err(_) => break,
        }
    }

    // Prefer AMD discrete GPU.
    for adapter in &adapters {
        let desc = unsafe { adapter.GetDesc1()? };

        if desc.Flags & DXGI_ADAPTER_FLAG_SOFTWARE.0 as u32 != 0 {
            continue;
        }

        // AMD PCI vendor ID.
        if desc.VendorId == 0x1002 {
            return unsafe { build_amd_info(adapter) };
        }
    }

    // No AMD GPU found.
    Ok(GpuInfo::empty())
}

#[cfg(windows)]
unsafe fn build_amd_info(
    adapter: &IDXGIAdapter1,
) -> windows::core::Result<GpuInfo> {
    let desc = unsafe { adapter.GetDesc1()? };

    let name = utf16_to_string(&desc.Description);

    let device_id = format!("{:04X}", desc.DeviceId);
    let vendor_id = format!("{:04X}", desc.VendorId);

    let is_rdna4 = is_rdna4_gpu(desc.DeviceId, &name);

    if !is_rdna4 {
        return Ok(GpuInfo {
            supported: false,
            name,
            vendor: "AMD".into(),
            vendor_id,
            device_id,
            architecture: "Unknown AMD architecture".into(),
            ..GpuInfo::empty()
        });
    }

    let architecture = determine_architecture(&name);
    let memory_bus = determine_memory_bus(&name);
    let memory_bandwidth = determine_memory_bandwidth(&name);

    let adapter3: IDXGIAdapter3 = adapter.cast()?;

    let mut memory =
        windows::Win32::Graphics::Dxgi::DXGI_QUERY_VIDEO_MEMORY_INFO::default();

    unsafe {
        adapter3.QueryVideoMemoryInfo(
            0,
            DXGI_MEMORY_SEGMENT_GROUP_LOCAL,
            &mut memory,
        )?;
    }

    let total_bytes = desc.DedicatedVideoMemory as u64;

    let usage_bytes = memory.CurrentUsage as u64;

    let budget_bytes = memory.Budget as u64;

    let available_bytes = budget_bytes.saturating_sub(usage_bytes);

    let vram_total = format_bytes(total_bytes);

    let vram_used = format_bytes(usage_bytes);

    let vram_available = format_bytes(available_bytes);

    Ok(GpuInfo {
        supported: true,

        name,
        architecture,
        vendor: "AMD".into(),
        vendor_id,
        device_id,

        pci_location: "Available through PCI/WMI backend in future version".into(),

        gpu_type: "Discrete GPU".into(),

        vram_total,
        vram_used,
        vram_available,

        memory_type: "GDDR6".into(),

        memory_bus,

        memory_bandwidth,

        gpu_clock: "N/A".into(),
        memory_clock: "N/A".into(),
        boost_clock: "N/A".into(),

        gpu_usage: "N/A".into(),
        temperature: "N/A".into(),
        power: "N/A".into(),
        fan: "N/A".into(),

        driver: "AMD Radeon Software".into(),
        driver_version: "N/A".into(),

        vbios: "N/A".into(),
    })
}

#[cfg(windows)]
fn is_rdna4_gpu(device_id: u32, name: &str) -> bool {
    /*
        Navi 48 / RX 9070 series.

        The device-ID list is deliberately conservative.
        GPU-X 0.1 is intended for RDNA 4 / RX 9000.

        We also check the name because AMD board partners can
        expose different device IDs.
    */

    const KNOWN_RDNA4_DEVICE_IDS: &[u32] = &[
        0x7550, // Navi 48
        0x7551,
        0x7552,
        0x7553,
        0x7554,
        0x7555,
    ];

    if KNOWN_RDNA4_DEVICE_IDS.contains(&device_id) {
        return true;
    }

    let name = name.to_ascii_lowercase();

    name.contains("rx 9000")
        || name.contains("rx 9070")
        || name.contains("rx 9060")
        || name.contains("radeon rx 90")
}

#[cfg(windows)]
fn determine_architecture(name: &str) -> String {
    let lower = name.to_ascii_lowercase();

    if lower.contains("9070") {
        "RDNA 4 / Navi 48".into()
    } else if lower.contains("9060") {
        "RDNA 4 / Navi 44".into()
    } else if lower.contains("9000") {
        "RDNA 4 / Radeon RX 9000".into()
    } else {
        "RDNA 4".into()
    }
}

#[cfg(windows)]
fn determine_memory_bus(name: &str) -> String {
    let lower = name.to_ascii_lowercase();

    if lower.contains("9070 xt") || lower.contains("9070") {
        "256-bit".into()
    } else if lower.contains("9060") {
        "128-bit".into()
    } else {
        "N/A".into()
    }
}

#[cfg(windows)]
fn determine_memory_bandwidth(name: &str) -> String {
    let lower = name.to_ascii_lowercase();

    if lower.contains("9070 xt") || lower.contains("9070") {
        "640 GB/s".into()
    } else if lower.contains("9060") {
        "288 GB/s".into()
    } else {
        "N/A".into()
    }
}

#[cfg(windows)]
fn utf16_to_string(buffer: &[u16]) -> String {
    let length = buffer
        .iter()
        .position(|&c| c == 0)
        .unwrap_or(buffer.len());

    String::from_utf16_lossy(&buffer[..length])
}

#[cfg(windows)]
fn format_bytes(bytes: u64) -> String {
    const GB: f64 = 1024.0 * 1024.0 * 1024.0;

    if bytes >= 1024 * 1024 * 1024 {
        format!("{:.2} GB", bytes as f64 / GB)
    } else if bytes >= 1024 * 1024 {
        format!("{:.2} MB", bytes as f64 / (1024.0 * 1024.0))
    } else if bytes >= 1024 {
        format!("{:.2} KB", bytes as f64 / 1024.0)
    } else {
        format!("{} B", bytes)
    }
}