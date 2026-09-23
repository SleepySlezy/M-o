#include <cstddef>
#include <cstdio>

#if defined(__has_include)

#if __has_include("ADLX/ADLXHelper.h")
    #include "ADLX/ADLXHelper.h"
    #define GPU_X_HAS_ADLX_HELPER 1
#elif __has_include("ADLXHelper.h")
    #include "ADLXHelper.h"
    #define GPU_X_HAS_ADLX_HELPER 1
#endif

#if __has_include("ADLX/ADLX.h")
    #include "ADLX/ADLX.h"
    #define GPU_X_HAS_ADLX_API 1
#elif __has_include("ADLX.h")
    #include "ADLX.h"
    #define GPU_X_HAS_ADLX_API 1
#endif

#if __has_include("ADLX/IPerformanceMonitoring.h")
    #include "ADLX/IPerformanceMonitoring.h"
    #define GPU_X_HAS_ADLX_PERF 1
#elif __has_include("IPerformanceMonitoring.h")
    #include "IPerformanceMonitoring.h"
    #define GPU_X_HAS_ADLX_PERF 1
#endif

#endif

#if defined(GPU_X_HAS_ADLX_HELPER) && \
    defined(GPU_X_HAS_ADLX_API) && \
    defined(GPU_X_HAS_ADLX_PERF)

using namespace adlx;
#define GPU_X_ADLX_AVAILABLE 1

#else

#define GPU_X_ADLX_AVAILABLE 0

#endif

struct GpuTelemetry
{
    double gpu_usage;
    double gpu_clock;
    double memory_clock;
    double temperature;
    double power;
    double fan_rpm;
};

extern "C" bool gpu_x_get_telemetry(
    int gpu_type,
    GpuTelemetry* out)
{
    if (out == nullptr)
        return false;

#if GPU_X_ADLX_AVAILABLE

    ADLXHelper helper;

    if (helper.Initialize() != ADLX_OK)
        return false;

    auto system = helper.GetSystemServices();

    if (system == nullptr)
    {
        helper.Terminate();
        return false;
    }

    IADLXGPUListPtr gpu_list;

    ADLX_RESULT result =
        system->GetGPUs(&gpu_list);

    if (result != ADLX_OK ||
        gpu_list == nullptr)
    {
        system = nullptr;
        helper.Terminate();
        return false;
    }

    IADLXGPUPtr gpu;

    adlx_uint count = gpu_list->Size();

    for (adlx_uint i = 0; i < count; ++i)
    {
        IADLXGPUPtr candidate;

        result = gpu_list->At(i, &candidate);

        if (result != ADLX_OK ||
            candidate == nullptr)
        {
            continue;
        }

        adlx_int type = GPUTYPE_UNDEFINED;

        result = candidate->Type(&type);

        if (result != ADLX_OK)
            continue;

        /*
            gpu_type:
            0 = Integrated
            1 = Discrete
        */

        if (gpu_type == 0 &&
            type == GPUTYPE_INTEGRATED)
        {
            gpu = candidate;
            break;
        }

        if (gpu_type == 1 &&
            type == GPUTYPE_DISCRETE)
        {
            gpu = candidate;
            break;
        }
    }

    if (gpu == nullptr)
    {
        gpu_list = nullptr;
        system = nullptr;
        helper.Terminate();
        return false;
    }

    IADLXPerformanceMonitoringServicesPtr performance;

    result =
        system->GetPerformanceMonitoringServices(
            &performance);

    if (result != ADLX_OK ||
        performance == nullptr)
    {
        performance = nullptr;
        gpu = nullptr;
        gpu_list = nullptr;
        system = nullptr;

        helper.Terminate();
        return false;
    }

    IADLXGPUMetricsPtr metrics;

    result =
        performance->GetCurrentGPUMetrics(
            gpu,
            &metrics);

    if (result != ADLX_OK ||
        metrics == nullptr)
    {
        metrics = nullptr;
        performance = nullptr;
        gpu = nullptr;
        gpu_list = nullptr;
        system = nullptr;

        helper.Terminate();
        return false;
    }

    adlx_double gpu_usage = 0.0;
    adlx_int gpu_clock = 0;
    adlx_int memory_clock = 0;
    adlx_double temperature = 0.0;
    adlx_double power = 0.0;
    adlx_int fan_rpm = 0;

    metrics->GPUUsage(&gpu_usage);
    metrics->GPUClockSpeed(&gpu_clock);
    metrics->GPUVRAMClockSpeed(&memory_clock);
    metrics->GPUTemperature(&temperature);
    metrics->GPUPower(&power);
    metrics->GPUFanSpeed(&fan_rpm);

    out->gpu_usage = gpu_usage;
    out->gpu_clock =
        static_cast<double>(gpu_clock);
    out->memory_clock =
        static_cast<double>(memory_clock);
    out->temperature = temperature;
    out->power = power;
    out->fan_rpm =
        static_cast<double>(fan_rpm);

    metrics = nullptr;
    performance = nullptr;
    gpu = nullptr;
    gpu_list = nullptr;
    system = nullptr;

    helper.Terminate();

    return true;

#else

    (void)gpu_type;
    (void)out;

    return false;

#endif
}