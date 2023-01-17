use crate::backends::{dx10_renderer_backend, dx9_renderer_backend};

pub enum BackendRendererUserData {
    None,
    Directx9(dx9_renderer_backend::DirectxData),
    Directx10(dx10_renderer_backend::DirectxData)
}
