use windows::Win32::Graphics::Direct3D9::{D3DPRESENT_PARAMETERS, IDirect3DSwapChain9};

// Helper structure we store in the void* RenderUserData field of each ImguiViewport to easily retrieve our backend data.
#[derive(Default,Debug,Clone)]
pub struct ViewportData
{
    pub swap_chain: Option<IDirect3DSwapChain9>,
    pub present_parameters: D3DPRESENT_PARAMETERS,
}
