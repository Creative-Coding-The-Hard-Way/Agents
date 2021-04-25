//! This module provides functions for picking a physical device and creating
//! the logical device.

mod physical_device;
mod queue;
mod queue_family_indices;

pub use self::{queue::Queue, queue_family_indices::QueueFamilyIndices};

use crate::graphics::vulkan::{ffi::to_os_ptrs, Instance, WindowSurface};
use physical_device::{
    pick_physical_device, required_device_extensions, required_device_features,
};

use anyhow::{Context, Result};
use ash::{
    version::{DeviceV1_0, InstanceV1_0},
    vk,
};
use std::{ffi::CString, sync::Arc};

/// This struct holds all device-specific resources, the physical device and
/// logical device for interacting with it, and the associated queues.
pub struct Device {
    pub physical_device: vk::PhysicalDevice,
    pub logical_device: ash::Device,

    pub graphics_queue: Arc<Queue>,
    pub present_queue: Arc<Queue>,

    /// The Vulkan library instance used to create this device
    pub instance: Arc<Instance>,
}

impl Device {
    /// Create a new device based on this application's required features and
    /// properties.
    pub fn new(window_surface: &dyn WindowSurface) -> Result<Arc<Device>> {
        let instance = window_surface.clone_vulkan_instance();
        let physical_device = pick_physical_device(&instance, window_surface)?;
        let queue_family_indices = QueueFamilyIndices::find(
            &physical_device,
            &instance.ash,
            window_surface,
        )?;
        let logical_device = create_logical_device(
            &instance,
            &physical_device,
            &queue_family_indices,
        )?;
        let (graphics_queue, present_queue) =
            queue_family_indices.get_queues(&logical_device)?;

        let device = Arc::new(Self {
            physical_device,
            logical_device,
            graphics_queue,
            present_queue,
            instance,
        });

        device.name_vulkan_object(
            "Application Logical Device",
            vk::ObjectType::DEVICE,
            &device.logical_device.handle(),
        )?;

        if device.graphics_queue.is_same(&device.present_queue) {
            device
                .graphics_queue
                .name_vulkan_object("graphics/present queue", &device)?;
        } else {
            device
                .graphics_queue
                .name_vulkan_object("graphics queue", &device)?;
            device
                .present_queue
                .name_vulkan_object("present queue", &device)?;
        }

        Ok(device)
    }

    /// Give a debug name for a vulkan object owned by this device.
    ///
    /// Whatever name is provided here will show up in the debug logs if there
    /// are any issues detected by the validation layers.
    pub fn name_vulkan_object<Name, Handle>(
        &self,
        name: Name,
        object_type: vk::ObjectType,
        handle: &Handle,
    ) -> Result<()>
    where
        Handle: vk::Handle + Copy,
        Name: Into<String>,
    {
        let cname = CString::new(name.into()).unwrap();

        let name_info = vk::DebugUtilsObjectNameInfoEXT::builder()
            .object_name(&cname)
            .object_type(object_type)
            .object_handle(handle.as_raw());

        unsafe {
            self.instance.debug.debug_utils_set_object_name(
                self.logical_device.handle(),
                &name_info,
            )?;
        }

        Ok(())
    }

    /// Directly allocate device memory.
    /// Unsafe because the caller is responsible for cleanup and handling the
    /// lifecycle of the allocated memory.
    pub unsafe fn allocate_memory(
        &self,
        memory_requirements: vk::MemoryRequirements,
        property_flags: vk::MemoryPropertyFlags,
    ) -> Result<vk::DeviceMemory> {
        let memory_properties = self
            .instance
            .ash
            .get_physical_device_memory_properties(self.physical_device);

        let memory_type_index = memory_properties
            .memory_types
            .iter()
            .enumerate()
            .find(|(i, memory_type)| {
                let type_supported =
                    memory_requirements.memory_type_bits & (1 << i) != 0;
                let properties_supported =
                    memory_type.property_flags.contains(property_flags);
                type_supported & properties_supported
            })
            .map(|(i, _memory_type)| i as u32)
            .with_context(|| {
                "unable to find a suitable memory type for this allocation!"
            })?;

        let allocate_info = vk::MemoryAllocateInfo::builder()
            .allocation_size(memory_requirements.size)
            .memory_type_index(memory_type_index);

        let memory =
            self.logical_device.allocate_memory(&allocate_info, None)?;

        Ok(memory)
    }

    /// Submit a command buffer to the specified queue, then wait for it to
    /// idle.
    pub unsafe fn submit_and_wait_idle(
        &self,
        queue: &Queue,
        command_buffer: vk::CommandBuffer,
    ) -> Result<()> {
        let queue_handle = queue.acquire();
        self.logical_device.queue_submit(
            *queue_handle,
            &[vk::SubmitInfo::builder()
                .command_buffers(&[command_buffer])
                .build()],
            vk::Fence::null(),
        )?;
        self.logical_device.queue_wait_idle(*queue_handle)?;
        Ok(())
    }
}

impl Drop for Device {
    /// Destroy the logical device.
    ///
    /// Device owns an Arc<Instance> so it's guaranteed that the instance will
    /// not be destroyed until the logical device has been dropped.
    fn drop(&mut self) {
        unsafe {
            self.logical_device.destroy_device(None);
        }
    }
}

/// Create a new logical device for use by this application. The caller is
/// responsible for destroying the device when done.
fn create_logical_device(
    instance: &Instance,
    physical_device: &vk::PhysicalDevice,
    queue_family_indices: &QueueFamilyIndices,
) -> Result<ash::Device> {
    let queue_create_infos = queue_family_indices.as_queue_create_infos();
    let features = required_device_features();
    let (_c_names, layer_name_ptrs) =
        unsafe { to_os_ptrs(&instance.enabled_layer_names) };
    let (_c_ext_names, ext_name_ptrs) =
        unsafe { to_os_ptrs(&required_device_extensions()) };

    let create_info = vk::DeviceCreateInfo::builder()
        .queue_create_infos(&queue_create_infos)
        .enabled_features(&features)
        .enabled_layer_names(&layer_name_ptrs)
        .enabled_extension_names(&ext_name_ptrs);

    let logical_device = unsafe {
        instance
            .ash
            .create_device(*physical_device, &create_info, None)
            .context("unable to create the logical device")?
    };

    Ok(logical_device)
}
