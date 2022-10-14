use crate::storage::ImGuiStorage;
use crate::utils::is_not_null;
use crate::window::ImGuiWindow;

pub unsafe fn SetStateStorage(tree: *mut ImGuiStorage)
{
    let mut window: *mut ImGuiWindow =  GimGui.CurrentWindow;
    window.DC.StateStorage = if is_not_null(tree) { tree } else { &window.StateStorage };
}

pub unsafe fn GetStateStorage() -> *mut ImGuiStorage
{
    let mut window: *mut ImGuiWindow =  GimGui.CurrentWindow;
    return window.DC.StateStorage;
}
