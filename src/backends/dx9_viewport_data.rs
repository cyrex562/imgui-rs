use windows::Win32::Graphics::Direct3D9::{D3DPRESENT_PARAMETERS, IDirect3DSwapChain9};

// Helper structure we store in the void* RenderUserData field of each ImGuiViewport to easily retrieve our backend data.
#[derive(Default,Debug,Clone)]
pub struct ImGui_ImplDX9_ViewportData
{
    pub swap_chain: *mut IDirect3DSwapChain9,
    // D3DPRESENT_PARAMETERS   d3dpp;
    pub d3dpp: D3DPRESENT_PARAMETERS,

    // ImGui_ImplDX9_ViewportData()  { SwapChain = NULL; ZeroMemory(&d3dpp, sizeof(D3DPRESENT_PARAMETERS)); }
    // ~ImGui_ImplDX9_ViewportData() { IM_ASSERT(SwapChain == NULL); }
}
