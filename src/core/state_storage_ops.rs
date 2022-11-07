use crate::core::storage::ImGuiStorage;
use crate::core::utils::is_not_null;
use crate::window::ImguiWindow;

pub unsafe fn SetStateStorage(tree: *mut ImGuiStorage) {
    let mut window = g.current_window_mut().unwrap();
    window.dc.StateStorage = if is_not_null(tree) {
        tree
    } else {
        &window.StateStorage
    };
}

pub unsafe fn GetStateStorage() -> *mut ImGuiStorage {
    let mut window = g.current_window_mut().unwrap();
    return window.dc.StateStorage;
}
