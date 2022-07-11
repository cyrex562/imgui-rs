/// Helper: Execute a block of code at maximum once a frame. Convenient if you want to quickly create an UI within deep-nested code that runs multiple times every frame.
/// Usage: static ImGuiOnceUponAFrame oaf; if (oaf) ImGui::Text("This will be called only once per frame");
#[derive(Default,Debug,Clone,PartialEq)]
pub struct ImGuiOnceUponAFrame
{
    pub ref_frame: i32,
    // ImGuiOnceUponAFrame() { ref_frame = -1; }
    // mutable int ref_frame;
    // operator bool() const { int current_frame = ImGui::GetFrameCount(); if (ref_frame == current_frame) return false; ref_frame = current_frame; return true; }
}

impl ImGuiOnceUponAFrame {
    pub fn new() -> Self {
        Self {
            ref_frame: -1,
        }
    }
}
