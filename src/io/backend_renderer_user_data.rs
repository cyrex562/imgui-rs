use crate::backends::{directx10, directx9};

pub enum BackendRendererUserData {
    None,
    Directx9(directx9::DirectxData),
    Directx10(directx10::DirectxData)
}
