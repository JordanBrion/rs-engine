use ash::version::InstanceV1_0;

pub unsafe fn search_physical_device_memory_type(
    instance: &ash::Instance,
    gpu: &ash::vk::PhysicalDevice,
    requirements: &ash::vk::MemoryRequirements,
    type_to_find: ash::vk::MemoryPropertyFlags,
) -> Result<usize, &'static str> {
    let memory_properties = instance.get_physical_device_memory_properties(*gpu);
    for (index, memory_type) in memory_properties.memory_types.iter().enumerate() {
        if requirements.memory_type_bits & (1 << index) > 0
            && memory_type.property_flags.contains(type_to_find)
        {
            return Ok(index);
        }
    }
    Err("Cannot find device memory type")
}
