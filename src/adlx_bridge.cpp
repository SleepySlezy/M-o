#if defined(__has_include)
    #if __has_include("ADLX/ADLXHelper.h") || \
        __has_include("ADLXHelper.h") || \
        __has_include("../ADLXHelper.h") || \
        __has_include("../include/ADLXHelper.h") || \
        __has_include("include/ADLXHelper.h")
        #define GPU_X_HAS_ADLX_HELPER 1
    #endif

    #if __has_include("ADLX/ADLX.h") || \
        __has_include("ADLX.h") || \
        __has_include("../ADLX.h") || \
        __has_include("../include/ADLX.h") || \
        __has_include("include/ADLX.h")
        #define GPU_X_HAS_ADLX_API 1
    #endif

    #if __has_include("ADLX/IPerformanceMonitoring.h") || \
        __has_include("IPerformanceMonitoring.h") || \
        __has_include("../IPerformanceMonitoring.h") || \
        __has_include("../include/IPerformanceMonitoring.h") || \
        __has_include("include/IPerformanceMonitoring.h")
        #define GPU_X_HAS_ADLX_PERF 1
    #endif
#endif

#if defined(GPU_X_HAS_ADLX_HELPER) && defined(GPU_X_HAS_ADLX_API) && defined(GPU_X_HAS_ADLX_PERF)
    #if defined(__has_include)
        #if __has_include("ADLX/ADLXHelper.h")
            #include "ADLX/ADLXHelper.h"
        #elif __has_include("ADLXHelper.h")
            #include "ADLXHelper.h"
        #elif __has_include("../ADLXHelper.h")
            #include "../ADLXHelper.h"
        #elif __has_include("../include/ADLXHelper.h")
            #include "../include/ADLXHelper.h"
        #elif __has_include("include/ADLXHelper.h")
            #include "include/ADLXHelper.h"
        #endif

        #if __has_include("ADLX/ADLX.h")
            #include "ADLX/ADLX.h"
        #elif __has_include("ADLX.h")
            #include "ADLX.h"
        #elif __has_include("../ADLX.h")
            #include "../ADLX.h"
        #elif __has_include("../include/ADLX.h")
            #include "../include/ADLX.h"
        #elif __has_include("include/ADLX.h")
            #include "include/ADLX.h"
        #endif

        #if __has_include("ADLX/IPerformanceMonitoring.h")
            #include "ADLX/IPerformanceMonitoring.h"
        #elif __has_include("IPerformanceMonitoring.h")
            #include "IPerformanceMonitoring.h"
        #elif __has_include("../IPerformanceMonitoring.h")
            #include "../IPerformanceMonitoring.h"
        #elif __has_include("../include/IPerformanceMonitoring.h")
            #include "../include/IPerformanceMonitoring.h"
        #elif __has_include("include/IPerformanceMonitoring.h")
            #include "include/IPerformanceMonitoring.h"
        #endif
    #else
        #include "ADLXHelper.h"
        #include "ADLX.h"
        #include "IPerformanceMonitoring.h"
    #endif

    using namespace adlx;
#else
    // ADLX is unavailable in this build; provide a conservative fallback so the
    // bridge still links and reports an invalid/empty state instead of crashing.
#endif

extern "C" {

struct GpuTelemetry {
    double gpu_usage;
    double gpu_clock;
    double memory_clock;
    double temperature;
    double power;
    double fan_rpm;
};

bool gpu_x_get_telemetry(GpuTelemetry* out)
{
    if (out == nullptr)
        return false;

    out->gpu_usage = -1.0;
    out->gpu_clock = -1.0;
    out->memory_clock = -1.0;
    out->temperature = -1.0;
    out->power = -1.0;
    out->fan_rpm = -1.0;

#if defined(GPU_X_HAS_ADLX_HELPER) && defined(GPU_X_HAS_ADLX_API) && defined(GPU_X_HAS_ADLX_PERF)
    ADLXHelper helper;

    if (helper.Initialize() != ADLX_OK)
        return false;

    auto system = helper.GetSystemServices();

    if (system == nullptr)
    {
        helper.Terminate();
        return false;
    }

    auto gpu_services = system->GetGPUsServices();

    if (gpu_services == nullptr)
    {
        helper.Terminate();
        return false;
    }

    IADLXGPUListPtr gpu_list;

    if (gpu_services->GetGPUs(&gpu_list) != ADLX_OK || gpu_list == nullptr)
    {
        helper.Terminate();
        return false;
    }

    IADLXGPUPtr gpu;

    if (gpu_list->At(0, &gpu) != ADLX_OK || gpu == nullptr)
    {
        helper.Terminate();
        return false;
    }

    auto performance =
        system->GetPerformanceMonitoringServices();

    if (performance == nullptr)
    {
        helper.Terminate();
        return false;
    }

    IADLXGPUMetricsPtr metrics;

    if (performance->GetCurrentGPUMetrics(
            gpu,
            &metrics) != ADLX_OK ||
        metrics == nullptr)
    {
        helper.Terminate();
        return false;
    }

    metrics->GPUUsage(&out->gpu_usage);

    int gpu_clock = -1;
    int memory_clock = -1;
    int fan_rpm = -1;

    metrics->GPUClockSpeed(&gpu_clock);
    metrics->GPUVRAMClockSpeed(&memory_clock);
    metrics->GPUTemperature(&out->temperature);
    metrics->GPUPower(&out->power);
    metrics->GPUFanSpeed(&fan_rpm);

    out->gpu_clock = static_cast<double>(gpu_clock);
    out->memory_clock = static_cast<double>(memory_clock);
    out->fan_rpm = static_cast<double>(fan_rpm);

    helper.Terminate();
    return true;
#else
    return false;
#endif
}

}