// This file is auto-generated by vulkano autogen from vk.xml header version 191.
// It should not be edited manually. Changes should be made by editing autogen.

pub struct EntryFunctions {
    pub v1_0: ash::vk::EntryFnV1_0,
    pub v1_1: ash::vk::EntryFnV1_1,
    pub v1_2: ash::vk::EntryFnV1_2,
}
impl EntryFunctions {
    pub fn load<F>(mut load_fn: F) -> EntryFunctions
    where
        F: FnMut(&CStr) -> *const c_void,
    {
        EntryFunctions {
            v1_0: ash::vk::EntryFnV1_0::load(&mut load_fn),
            v1_1: ash::vk::EntryFnV1_1::load(&mut load_fn),
            v1_2: ash::vk::EntryFnV1_2::load(&mut load_fn),
        }
    }
}
pub struct InstanceFunctions {
    pub v1_0: ash::vk::InstanceFnV1_0,
    pub v1_1: ash::vk::InstanceFnV1_1,
    pub v1_2: ash::vk::InstanceFnV1_2,
    pub khr_android_surface: ash::vk::KhrAndroidSurfaceFn,
    pub khr_device_group_creation: ash::vk::KhrDeviceGroupCreationFn,
    pub khr_display: ash::vk::KhrDisplayFn,
    pub khr_external_fence_capabilities: ash::vk::KhrExternalFenceCapabilitiesFn,
    pub khr_external_memory_capabilities: ash::vk::KhrExternalMemoryCapabilitiesFn,
    pub khr_external_semaphore_capabilities: ash::vk::KhrExternalSemaphoreCapabilitiesFn,
    pub khr_get_display_properties2: ash::vk::KhrGetDisplayProperties2Fn,
    pub khr_get_physical_device_properties2: ash::vk::KhrGetPhysicalDeviceProperties2Fn,
    pub khr_get_surface_capabilities2: ash::vk::KhrGetSurfaceCapabilities2Fn,
    pub khr_surface: ash::vk::KhrSurfaceFn,
    pub khr_wayland_surface: ash::vk::KhrWaylandSurfaceFn,
    pub khr_win32_surface: ash::vk::KhrWin32SurfaceFn,
    pub khr_xcb_surface: ash::vk::KhrXcbSurfaceFn,
    pub khr_xlib_surface: ash::vk::KhrXlibSurfaceFn,
    pub ext_acquire_drm_display: ash::vk::ExtAcquireDrmDisplayFn,
    pub ext_acquire_xlib_display: ash::vk::ExtAcquireXlibDisplayFn,
    pub ext_debug_report: ash::vk::ExtDebugReportFn,
    pub ext_debug_utils: ash::vk::ExtDebugUtilsFn,
    pub ext_direct_mode_display: ash::vk::ExtDirectModeDisplayFn,
    pub ext_directfb_surface: ash::vk::ExtDirectfbSurfaceFn,
    pub ext_display_surface_counter: ash::vk::ExtDisplaySurfaceCounterFn,
    pub ext_headless_surface: ash::vk::ExtHeadlessSurfaceFn,
    pub ext_metal_surface: ash::vk::ExtMetalSurfaceFn,
    pub fuchsia_imagepipe_surface: ash::vk::FuchsiaImagepipeSurfaceFn,
    pub ggp_stream_descriptor_surface: ash::vk::GgpStreamDescriptorSurfaceFn,
    pub mvk_ios_surface: ash::vk::MvkIosSurfaceFn,
    pub mvk_macos_surface: ash::vk::MvkMacosSurfaceFn,
    pub nn_vi_surface: ash::vk::NnViSurfaceFn,
    pub nv_external_memory_capabilities: ash::vk::NvExternalMemoryCapabilitiesFn,
    pub qnx_screen_surface: ash::vk::QnxScreenSurfaceFn,
}
impl InstanceFunctions {
    pub fn load<F>(mut load_fn: F) -> InstanceFunctions
    where
        F: FnMut(&CStr) -> *const c_void,
    {
        InstanceFunctions {
            v1_0: ash::vk::InstanceFnV1_0::load(&mut load_fn),
            v1_1: ash::vk::InstanceFnV1_1::load(&mut load_fn),
            v1_2: ash::vk::InstanceFnV1_2::load(&mut load_fn),
            khr_android_surface: ash::vk::KhrAndroidSurfaceFn::load(&mut load_fn),
            khr_device_group_creation: ash::vk::KhrDeviceGroupCreationFn::load(&mut load_fn),
            khr_display: ash::vk::KhrDisplayFn::load(&mut load_fn),
            khr_external_fence_capabilities: ash::vk::KhrExternalFenceCapabilitiesFn::load(
                &mut load_fn,
            ),
            khr_external_memory_capabilities: ash::vk::KhrExternalMemoryCapabilitiesFn::load(
                &mut load_fn,
            ),
            khr_external_semaphore_capabilities: ash::vk::KhrExternalSemaphoreCapabilitiesFn::load(
                &mut load_fn,
            ),
            khr_get_display_properties2: ash::vk::KhrGetDisplayProperties2Fn::load(&mut load_fn),
            khr_get_physical_device_properties2: ash::vk::KhrGetPhysicalDeviceProperties2Fn::load(
                &mut load_fn,
            ),
            khr_get_surface_capabilities2: ash::vk::KhrGetSurfaceCapabilities2Fn::load(
                &mut load_fn,
            ),
            khr_surface: ash::vk::KhrSurfaceFn::load(&mut load_fn),
            khr_wayland_surface: ash::vk::KhrWaylandSurfaceFn::load(&mut load_fn),
            khr_win32_surface: ash::vk::KhrWin32SurfaceFn::load(&mut load_fn),
            khr_xcb_surface: ash::vk::KhrXcbSurfaceFn::load(&mut load_fn),
            khr_xlib_surface: ash::vk::KhrXlibSurfaceFn::load(&mut load_fn),
            ext_acquire_drm_display: ash::vk::ExtAcquireDrmDisplayFn::load(&mut load_fn),
            ext_acquire_xlib_display: ash::vk::ExtAcquireXlibDisplayFn::load(&mut load_fn),
            ext_debug_report: ash::vk::ExtDebugReportFn::load(&mut load_fn),
            ext_debug_utils: ash::vk::ExtDebugUtilsFn::load(&mut load_fn),
            ext_direct_mode_display: ash::vk::ExtDirectModeDisplayFn::load(&mut load_fn),
            ext_directfb_surface: ash::vk::ExtDirectfbSurfaceFn::load(&mut load_fn),
            ext_display_surface_counter: ash::vk::ExtDisplaySurfaceCounterFn::load(&mut load_fn),
            ext_headless_surface: ash::vk::ExtHeadlessSurfaceFn::load(&mut load_fn),
            ext_metal_surface: ash::vk::ExtMetalSurfaceFn::load(&mut load_fn),
            fuchsia_imagepipe_surface: ash::vk::FuchsiaImagepipeSurfaceFn::load(&mut load_fn),
            ggp_stream_descriptor_surface: ash::vk::GgpStreamDescriptorSurfaceFn::load(
                &mut load_fn,
            ),
            mvk_ios_surface: ash::vk::MvkIosSurfaceFn::load(&mut load_fn),
            mvk_macos_surface: ash::vk::MvkMacosSurfaceFn::load(&mut load_fn),
            nn_vi_surface: ash::vk::NnViSurfaceFn::load(&mut load_fn),
            nv_external_memory_capabilities: ash::vk::NvExternalMemoryCapabilitiesFn::load(
                &mut load_fn,
            ),
            qnx_screen_surface: ash::vk::QnxScreenSurfaceFn::load(&mut load_fn),
        }
    }
}
pub struct DeviceFunctions {
    pub v1_0: ash::vk::DeviceFnV1_0,
    pub v1_1: ash::vk::DeviceFnV1_1,
    pub v1_2: ash::vk::DeviceFnV1_2,
    pub khr_acceleration_structure: ash::vk::KhrAccelerationStructureFn,
    pub khr_bind_memory2: ash::vk::KhrBindMemory2Fn,
    pub khr_buffer_device_address: ash::vk::KhrBufferDeviceAddressFn,
    pub khr_copy_commands2: ash::vk::KhrCopyCommands2Fn,
    pub khr_create_renderpass2: ash::vk::KhrCreateRenderpass2Fn,
    pub khr_deferred_host_operations: ash::vk::KhrDeferredHostOperationsFn,
    pub khr_descriptor_update_template: ash::vk::KhrDescriptorUpdateTemplateFn,
    pub khr_device_group: ash::vk::KhrDeviceGroupFn,
    pub khr_display_swapchain: ash::vk::KhrDisplaySwapchainFn,
    pub khr_draw_indirect_count: ash::vk::KhrDrawIndirectCountFn,
    pub khr_external_fence_fd: ash::vk::KhrExternalFenceFdFn,
    pub khr_external_fence_win32: ash::vk::KhrExternalFenceWin32Fn,
    pub khr_external_memory_fd: ash::vk::KhrExternalMemoryFdFn,
    pub khr_external_memory_win32: ash::vk::KhrExternalMemoryWin32Fn,
    pub khr_external_semaphore_fd: ash::vk::KhrExternalSemaphoreFdFn,
    pub khr_external_semaphore_win32: ash::vk::KhrExternalSemaphoreWin32Fn,
    pub khr_fragment_shading_rate: ash::vk::KhrFragmentShadingRateFn,
    pub khr_get_memory_requirements2: ash::vk::KhrGetMemoryRequirements2Fn,
    pub khr_maintenance1: ash::vk::KhrMaintenance1Fn,
    pub khr_maintenance3: ash::vk::KhrMaintenance3Fn,
    pub khr_performance_query: ash::vk::KhrPerformanceQueryFn,
    pub khr_pipeline_executable_properties: ash::vk::KhrPipelineExecutablePropertiesFn,
    pub khr_present_wait: ash::vk::KhrPresentWaitFn,
    pub khr_push_descriptor: ash::vk::KhrPushDescriptorFn,
    pub khr_ray_tracing_pipeline: ash::vk::KhrRayTracingPipelineFn,
    pub khr_sampler_ycbcr_conversion: ash::vk::KhrSamplerYcbcrConversionFn,
    pub khr_shared_presentable_image: ash::vk::KhrSharedPresentableImageFn,
    pub khr_swapchain: ash::vk::KhrSwapchainFn,
    pub khr_synchronization2: ash::vk::KhrSynchronization2Fn,
    pub khr_timeline_semaphore: ash::vk::KhrTimelineSemaphoreFn,
    pub khr_video_decode_queue: ash::vk::KhrVideoDecodeQueueFn,
    pub khr_video_encode_queue: ash::vk::KhrVideoEncodeQueueFn,
    pub khr_video_queue: ash::vk::KhrVideoQueueFn,
    pub ext_buffer_device_address: ash::vk::ExtBufferDeviceAddressFn,
    pub ext_calibrated_timestamps: ash::vk::ExtCalibratedTimestampsFn,
    pub ext_color_write_enable: ash::vk::ExtColorWriteEnableFn,
    pub ext_conditional_rendering: ash::vk::ExtConditionalRenderingFn,
    pub ext_debug_marker: ash::vk::ExtDebugMarkerFn,
    pub ext_discard_rectangles: ash::vk::ExtDiscardRectanglesFn,
    pub ext_display_control: ash::vk::ExtDisplayControlFn,
    pub ext_extended_dynamic_state: ash::vk::ExtExtendedDynamicStateFn,
    pub ext_extended_dynamic_state2: ash::vk::ExtExtendedDynamicState2Fn,
    pub ext_external_memory_host: ash::vk::ExtExternalMemoryHostFn,
    pub ext_full_screen_exclusive: ash::vk::ExtFullScreenExclusiveFn,
    pub ext_hdr_metadata: ash::vk::ExtHdrMetadataFn,
    pub ext_host_query_reset: ash::vk::ExtHostQueryResetFn,
    pub ext_image_drm_format_modifier: ash::vk::ExtImageDrmFormatModifierFn,
    pub ext_line_rasterization: ash::vk::ExtLineRasterizationFn,
    pub ext_multi_draw: ash::vk::ExtMultiDrawFn,
    pub ext_pageable_device_local_memory: ash::vk::ExtPageableDeviceLocalMemoryFn,
    pub ext_private_data: ash::vk::ExtPrivateDataFn,
    pub ext_sample_locations: ash::vk::ExtSampleLocationsFn,
    pub ext_tooling_info: ash::vk::ExtToolingInfoFn,
    pub ext_transform_feedback: ash::vk::ExtTransformFeedbackFn,
    pub ext_validation_cache: ash::vk::ExtValidationCacheFn,
    pub ext_vertex_input_dynamic_state: ash::vk::ExtVertexInputDynamicStateFn,
    pub amd_buffer_marker: ash::vk::AmdBufferMarkerFn,
    pub amd_display_native_hdr: ash::vk::AmdDisplayNativeHdrFn,
    pub amd_draw_indirect_count: ash::vk::AmdDrawIndirectCountFn,
    pub amd_shader_info: ash::vk::AmdShaderInfoFn,
    pub android_external_memory_android_hardware_buffer:
        ash::vk::AndroidExternalMemoryAndroidHardwareBufferFn,
    pub fuchsia_external_memory: ash::vk::FuchsiaExternalMemoryFn,
    pub fuchsia_external_semaphore: ash::vk::FuchsiaExternalSemaphoreFn,
    pub google_display_timing: ash::vk::GoogleDisplayTimingFn,
    pub huawei_invocation_mask: ash::vk::HuaweiInvocationMaskFn,
    pub huawei_subpass_shading: ash::vk::HuaweiSubpassShadingFn,
    pub intel_performance_query: ash::vk::IntelPerformanceQueryFn,
    pub nvx_binary_import: ash::vk::NvxBinaryImportFn,
    pub nvx_image_view_handle: ash::vk::NvxImageViewHandleFn,
    pub nv_acquire_winrt_display: ash::vk::NvAcquireWinrtDisplayFn,
    pub nv_clip_space_w_scaling: ash::vk::NvClipSpaceWScalingFn,
    pub nv_cooperative_matrix: ash::vk::NvCooperativeMatrixFn,
    pub nv_coverage_reduction_mode: ash::vk::NvCoverageReductionModeFn,
    pub nv_device_diagnostic_checkpoints: ash::vk::NvDeviceDiagnosticCheckpointsFn,
    pub nv_device_generated_commands: ash::vk::NvDeviceGeneratedCommandsFn,
    pub nv_external_memory_rdma: ash::vk::NvExternalMemoryRdmaFn,
    pub nv_external_memory_win32: ash::vk::NvExternalMemoryWin32Fn,
    pub nv_fragment_shading_rate_enums: ash::vk::NvFragmentShadingRateEnumsFn,
    pub nv_mesh_shader: ash::vk::NvMeshShaderFn,
    pub nv_ray_tracing: ash::vk::NvRayTracingFn,
    pub nv_scissor_exclusive: ash::vk::NvScissorExclusiveFn,
    pub nv_shading_rate_image: ash::vk::NvShadingRateImageFn,
}
impl DeviceFunctions {
    pub fn load<F>(mut load_fn: F) -> DeviceFunctions
    where
        F: FnMut(&CStr) -> *const c_void,
    {
        DeviceFunctions {
            v1_0: ash::vk::DeviceFnV1_0::load(&mut load_fn),
            v1_1: ash::vk::DeviceFnV1_1::load(&mut load_fn),
            v1_2: ash::vk::DeviceFnV1_2::load(&mut load_fn),
            khr_acceleration_structure: ash::vk::KhrAccelerationStructureFn::load(&mut load_fn),
            khr_bind_memory2: ash::vk::KhrBindMemory2Fn::load(&mut load_fn),
            khr_buffer_device_address: ash::vk::KhrBufferDeviceAddressFn::load(&mut load_fn),
            khr_copy_commands2: ash::vk::KhrCopyCommands2Fn::load(&mut load_fn),
            khr_create_renderpass2: ash::vk::KhrCreateRenderpass2Fn::load(&mut load_fn),
            khr_deferred_host_operations: ash::vk::KhrDeferredHostOperationsFn::load(&mut load_fn),
            khr_descriptor_update_template: ash::vk::KhrDescriptorUpdateTemplateFn::load(
                &mut load_fn,
            ),
            khr_device_group: ash::vk::KhrDeviceGroupFn::load(&mut load_fn),
            khr_display_swapchain: ash::vk::KhrDisplaySwapchainFn::load(&mut load_fn),
            khr_draw_indirect_count: ash::vk::KhrDrawIndirectCountFn::load(&mut load_fn),
            khr_external_fence_fd: ash::vk::KhrExternalFenceFdFn::load(&mut load_fn),
            khr_external_fence_win32: ash::vk::KhrExternalFenceWin32Fn::load(&mut load_fn),
            khr_external_memory_fd: ash::vk::KhrExternalMemoryFdFn::load(&mut load_fn),
            khr_external_memory_win32: ash::vk::KhrExternalMemoryWin32Fn::load(&mut load_fn),
            khr_external_semaphore_fd: ash::vk::KhrExternalSemaphoreFdFn::load(&mut load_fn),
            khr_external_semaphore_win32: ash::vk::KhrExternalSemaphoreWin32Fn::load(&mut load_fn),
            khr_fragment_shading_rate: ash::vk::KhrFragmentShadingRateFn::load(&mut load_fn),
            khr_get_memory_requirements2: ash::vk::KhrGetMemoryRequirements2Fn::load(&mut load_fn),
            khr_maintenance1: ash::vk::KhrMaintenance1Fn::load(&mut load_fn),
            khr_maintenance3: ash::vk::KhrMaintenance3Fn::load(&mut load_fn),
            khr_performance_query: ash::vk::KhrPerformanceQueryFn::load(&mut load_fn),
            khr_pipeline_executable_properties: ash::vk::KhrPipelineExecutablePropertiesFn::load(
                &mut load_fn,
            ),
            khr_present_wait: ash::vk::KhrPresentWaitFn::load(&mut load_fn),
            khr_push_descriptor: ash::vk::KhrPushDescriptorFn::load(&mut load_fn),
            khr_ray_tracing_pipeline: ash::vk::KhrRayTracingPipelineFn::load(&mut load_fn),
            khr_sampler_ycbcr_conversion: ash::vk::KhrSamplerYcbcrConversionFn::load(&mut load_fn),
            khr_shared_presentable_image: ash::vk::KhrSharedPresentableImageFn::load(&mut load_fn),
            khr_swapchain: ash::vk::KhrSwapchainFn::load(&mut load_fn),
            khr_synchronization2: ash::vk::KhrSynchronization2Fn::load(&mut load_fn),
            khr_timeline_semaphore: ash::vk::KhrTimelineSemaphoreFn::load(&mut load_fn),
            khr_video_decode_queue: ash::vk::KhrVideoDecodeQueueFn::load(&mut load_fn),
            khr_video_encode_queue: ash::vk::KhrVideoEncodeQueueFn::load(&mut load_fn),
            khr_video_queue: ash::vk::KhrVideoQueueFn::load(&mut load_fn),
            ext_buffer_device_address: ash::vk::ExtBufferDeviceAddressFn::load(&mut load_fn),
            ext_calibrated_timestamps: ash::vk::ExtCalibratedTimestampsFn::load(&mut load_fn),
            ext_color_write_enable: ash::vk::ExtColorWriteEnableFn::load(&mut load_fn),
            ext_conditional_rendering: ash::vk::ExtConditionalRenderingFn::load(&mut load_fn),
            ext_debug_marker: ash::vk::ExtDebugMarkerFn::load(&mut load_fn),
            ext_discard_rectangles: ash::vk::ExtDiscardRectanglesFn::load(&mut load_fn),
            ext_display_control: ash::vk::ExtDisplayControlFn::load(&mut load_fn),
            ext_extended_dynamic_state: ash::vk::ExtExtendedDynamicStateFn::load(&mut load_fn),
            ext_extended_dynamic_state2: ash::vk::ExtExtendedDynamicState2Fn::load(&mut load_fn),
            ext_external_memory_host: ash::vk::ExtExternalMemoryHostFn::load(&mut load_fn),
            ext_full_screen_exclusive: ash::vk::ExtFullScreenExclusiveFn::load(&mut load_fn),
            ext_hdr_metadata: ash::vk::ExtHdrMetadataFn::load(&mut load_fn),
            ext_host_query_reset: ash::vk::ExtHostQueryResetFn::load(&mut load_fn),
            ext_image_drm_format_modifier: ash::vk::ExtImageDrmFormatModifierFn::load(&mut load_fn),
            ext_line_rasterization: ash::vk::ExtLineRasterizationFn::load(&mut load_fn),
            ext_multi_draw: ash::vk::ExtMultiDrawFn::load(&mut load_fn),
            ext_pageable_device_local_memory: ash::vk::ExtPageableDeviceLocalMemoryFn::load(
                &mut load_fn,
            ),
            ext_private_data: ash::vk::ExtPrivateDataFn::load(&mut load_fn),
            ext_sample_locations: ash::vk::ExtSampleLocationsFn::load(&mut load_fn),
            ext_tooling_info: ash::vk::ExtToolingInfoFn::load(&mut load_fn),
            ext_transform_feedback: ash::vk::ExtTransformFeedbackFn::load(&mut load_fn),
            ext_validation_cache: ash::vk::ExtValidationCacheFn::load(&mut load_fn),
            ext_vertex_input_dynamic_state: ash::vk::ExtVertexInputDynamicStateFn::load(
                &mut load_fn,
            ),
            amd_buffer_marker: ash::vk::AmdBufferMarkerFn::load(&mut load_fn),
            amd_display_native_hdr: ash::vk::AmdDisplayNativeHdrFn::load(&mut load_fn),
            amd_draw_indirect_count: ash::vk::AmdDrawIndirectCountFn::load(&mut load_fn),
            amd_shader_info: ash::vk::AmdShaderInfoFn::load(&mut load_fn),
            android_external_memory_android_hardware_buffer:
                ash::vk::AndroidExternalMemoryAndroidHardwareBufferFn::load(&mut load_fn),
            fuchsia_external_memory: ash::vk::FuchsiaExternalMemoryFn::load(&mut load_fn),
            fuchsia_external_semaphore: ash::vk::FuchsiaExternalSemaphoreFn::load(&mut load_fn),
            google_display_timing: ash::vk::GoogleDisplayTimingFn::load(&mut load_fn),
            huawei_invocation_mask: ash::vk::HuaweiInvocationMaskFn::load(&mut load_fn),
            huawei_subpass_shading: ash::vk::HuaweiSubpassShadingFn::load(&mut load_fn),
            intel_performance_query: ash::vk::IntelPerformanceQueryFn::load(&mut load_fn),
            nvx_binary_import: ash::vk::NvxBinaryImportFn::load(&mut load_fn),
            nvx_image_view_handle: ash::vk::NvxImageViewHandleFn::load(&mut load_fn),
            nv_acquire_winrt_display: ash::vk::NvAcquireWinrtDisplayFn::load(&mut load_fn),
            nv_clip_space_w_scaling: ash::vk::NvClipSpaceWScalingFn::load(&mut load_fn),
            nv_cooperative_matrix: ash::vk::NvCooperativeMatrixFn::load(&mut load_fn),
            nv_coverage_reduction_mode: ash::vk::NvCoverageReductionModeFn::load(&mut load_fn),
            nv_device_diagnostic_checkpoints: ash::vk::NvDeviceDiagnosticCheckpointsFn::load(
                &mut load_fn,
            ),
            nv_device_generated_commands: ash::vk::NvDeviceGeneratedCommandsFn::load(&mut load_fn),
            nv_external_memory_rdma: ash::vk::NvExternalMemoryRdmaFn::load(&mut load_fn),
            nv_external_memory_win32: ash::vk::NvExternalMemoryWin32Fn::load(&mut load_fn),
            nv_fragment_shading_rate_enums: ash::vk::NvFragmentShadingRateEnumsFn::load(
                &mut load_fn,
            ),
            nv_mesh_shader: ash::vk::NvMeshShaderFn::load(&mut load_fn),
            nv_ray_tracing: ash::vk::NvRayTracingFn::load(&mut load_fn),
            nv_scissor_exclusive: ash::vk::NvScissorExclusiveFn::load(&mut load_fn),
            nv_shading_rate_image: ash::vk::NvShadingRateImageFn::load(&mut load_fn),
        }
    }
}
