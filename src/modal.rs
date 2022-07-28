use crate::{Context, INVALID_ID};
use crate::window::{Window, WindowFlags};

// When a modal popup is open, newly created windows that want focus (i.e. are not popups and do not specify ImGuiWindowFlags_NoFocusOnAppearing)
// should be positioned behind that modal window, unless the window was created inside the modal begin-stack.
// In case of multiple stacked modals newly created window honors begin stack order and does not go below its own modal parent.
// - window             // FindBlockingModal() returns Modal1
//   - window           //                  .. returns Modal1
//   - Modal1           //                  .. returns Modal2
//      - window        //                  .. returns Modal2
//          - window    //                  .. returns Modal2
//          - Modal2    //                  .. returns Modal2
// static ImGuiWindow* ImGui::FindBlockingModal(ImGuiWindow* window)
pub fn find_blocking_modal(g: &mut Context, window: &mut Window) -> Option<&mut Window>
{
    // ImGuiContext& g = *GImGui;
    // if (g.open_popup_stack.size <= 0)
    //     return None;
    if g.open_popup_stack.is_empty() {
        return None;
    }

    // Find a modal that has common parent with specified window. Specified window should be positioned behind that modal.
    // for (int i = g.open_popup_stack.size - 1; i >= 0; i--)
    for i in g.open_popup_stack.len() - 1 .. 0
    {
        // ImGuiWindow* popup_window = g.open_popup_stack.data[i].window;
        let psd = &mut g.open_popup_stack[i];
        let popup_window = g.get_window(psd.window_id)

        // if popup_window.is_none() || !(popup_window.unwrap().flags.contains(&WindowFlags::Modal)) {
        //     continue;
        // }
        if popup_window.is_none() {
            continue;
        }

        let popup_window_obj = popup_window.unwrap();
        if popup_window_obj.flags.contains(&WindowFlags::Modal) {
            continue;
        }

        if !popup_window_obj.active && !popup_window_obj.was_active {    // Check was_active, because this code may run before popup renders on current frame, also check active to handle newly created windows.
            continue;
        }
        if is_window_within_begin_stack_of(window, popup_window) {       // window is rendered over last modal, no render order change needed.
            break;
        }
        // for (ImGuiWindow* parent = popup_window.ParentWindowInBeginStack.root_window; parent != None; parent = parent.ParentWindowInBeginStack.root_window)
        let mut parent_window_in_begin_stack = g.get_window(popup_window_obj.parent_window_in_begin_stack_id).unwrap();
        while parent_window_in_begin_stack.root_window_id != INVALID_ID {
            let parent_win = g.get_window(parent_window_in_begin_stack.root_window_id).unwrap();
            if is_window_within_begin_stack_of(window, parent_win) {
                return Some(popup_window_obj);
            }                           // Place window above its begin stack parent.
            parent_window_in_begin_stack = g.get_window(parent_win.parent_window_in_begin_stack_id).unwrap();
        }
    }
    return None;
}
