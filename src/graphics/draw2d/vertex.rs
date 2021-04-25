use ash::vk;
use memoffset::offset_of;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub pos: [f32; 2],
    pub uv: [f32; 2],
    pub rgba: [f32; 4],
}

impl Default for Vertex {
    fn default() -> Self {
        Self {
            pos: [0.0, 0.0],
            uv: [0.0, 0.0],
            rgba: [1.0, 1.0, 1.0, 1.0],
        }
    }
}

impl Vertex {
    /// Build a binding description for this vertex type.
    pub fn binding_description() -> (
        Vec<vk::VertexInputBindingDescription>,
        Vec<vk::VertexInputAttributeDescription>,
    ) {
        let binding = vk::VertexInputBindingDescription::builder()
            .binding(0)
            .stride(std::mem::size_of::<Self>() as u32)
            .input_rate(vk::VertexInputRate::VERTEX)
            .build();
        let pos = vk::VertexInputAttributeDescription::builder()
            .binding(0)
            .location(0)
            .format(vk::Format::R32G32_SFLOAT)
            .offset(offset_of!(Vertex, pos) as u32)
            .build();
        let uv = vk::VertexInputAttributeDescription::builder()
            .binding(0)
            .location(1)
            .format(vk::Format::R32G32_SFLOAT)
            .offset(offset_of!(Vertex, uv) as u32)
            .build();
        let rgba = vk::VertexInputAttributeDescription::builder()
            .binding(0)
            .location(2)
            .format(vk::Format::R32G32B32A32_SFLOAT)
            .offset(offset_of!(Vertex, rgba) as u32)
            .build();
        (vec![binding], vec![pos, uv, rgba])
    }
}
