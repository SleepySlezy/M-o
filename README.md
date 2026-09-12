# PLEASE NOTE YOU NEED TO RUN `cargo check` & only `cargo run --release` if check responds with "`Finished `dev` profile`"

## What this first version actually gets

For the **RX 9070 XT**, the important part is the DXGI adapter information. DXGI exposes the adapter's dedicated video-memory amount, and `IDXGIAdapter3::QueryVideoMemoryInfo` provides current memory usage/budget information.

So the initial application should give you roughly:

```text
GPU
Name             AMD Radeon RX 9070 XT
Architecture     RDNA 4 / Navi 48
Vendor           AMD
PCI Vendor ID    1002
PCI Device ID    7550
GPU Type         Discrete GPU

Memory
VRAM             16.00 GB
VRAM Used        X.XX GB
VRAM Available   X.XX GB
Memory Type      GDDR6
Memory Bus       256-bit
Bandwidth        640 GB/s
