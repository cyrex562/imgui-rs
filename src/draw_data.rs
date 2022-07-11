use crate::draw_list::DrawList;
use crate::types::Id32;

/// All draw data to render a Dear ImGui frame
/// (NB: the style and the naming convention here is a little inconsistent, we currently preserve them for backward compatibility purpose,
/// as this is one of the oldest structure exposed by the library! Basically, ImDrawList == CmdList)
#[derive(Debug,Clone,Default)]
pub struct DrawData
{
    pub valid: bool,                  // Only valid after Render() is called and before the next NewFrame() is called.
    pub cmd_lists_count: i32,        // Number of ImDrawList* to render
    pub total_idx_count: i32,        // For convenience, sum of all ImDrawList's idx_buffer.size
    pub total_vtx_count: i32,        // For convenience, sum of all ImDrawList's vtx_buffer.size
    // ImDrawList**    cmd_lists;               // Array of ImDrawList* to render. The ImDrawList are owned by ImGuiContext and only pointed to from here.
    pub cmd_lists: Vec<DrawList>,
    pub display_pos: Vector2D,             // Top-left position of the viewport to render (== top-left of the orthogonal projection matrix to use) (== GetMainViewport()->pos for the main viewport, == (0.0) in most single-viewport applications)
    pub display_size: Vector2D,            // size of the viewport to render (== GetMainViewport()->size for the main viewport, == io.display_size in most single-viewport applications)
    pub framebuffer_scale: Vector2D,       // Amount of pixels for each unit of display_size. Based on io.display_framebuffer_scale. Generally (1,1) on normal display, (2,2) on OSX with Retina display.
    // ImGuiViewport*  OwnerViewport;          // viewport carrying the ImDrawData instance, might be of use to the renderer (generally not).
    pub owner_viewport: Id32,

}

impl DrawData {
    // // Functions
    //     ImDrawData()    { clear(); }
    //     void clear()    { memset(this, 0, sizeof(*this)); }     // The ImDrawList are owned by ImGuiContext!
    pub fn clear(&mut self) {
        self.valid = false;
        self.cmd_lists_count = 0;
        self.total_idx_count = 0;
        self.total_vtx_count = 0;
        self.cmd_lists.clear();
        self.display_pos.clear();
        self.display_size.clear();
        self.framebuffer_scale.clear();
        self.owner_viewport.clear();
    }
    //      void  de_index_all_buffers();                    // Helper to convert all buffers from indexed to non-indexed, in case you cannot render indexed. Note: this is slow and most likely a waste of resources. Always prefer indexed rendering!
    pub fn de_index_all_buffers(&mut self) {
        todo!()
    }
    //      void  scale_clip_rects(const Vector2D& fb_scale); // Helper to scale the clip_rect field of each ImDrawCmd. Use if your final output buffer is at a different scale than Dear ImGui expects, or if there is a difference between your window resolution and framebuffer resolution.
    pub fn scale_clip_rects(&mut self, fb_scale: &Vector2D) {
        todo!()
    }
}
