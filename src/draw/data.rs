use crate::context::Context;
use crate::draw::list::{add_draw_list_to_draw_data, DrawList};
use crate::draw::vertex::DrawVertex;
use crate::types::{DrawIndex, Id32};
use crate::vectors::vector_2d::Vector2D;
use crate::vectors::Vector4D;
use crate::window::{checks, get, Window, WindowFlags};

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
    pub cmd_lists: Vec<Id32>,
    pub display_pos: Vector2D,             // Top-left position of the viewport to render (== top-left of the orthogonal projection matrix to use) (== get_main_viewport()->pos for the main viewport, == (0.0) in most single-viewport applications)
    pub display_size: Vector2D,            // size of the viewport to render (== get_main_viewport()->size for the main viewport, == io.display_size in most single-viewport applications)
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
        self.owner_viewport = 0;
    }
    //      void  de_index_all_buffers();                    // Helper to convert all buffers from indexed to non-indexed, in case you cannot render indexed. Note: this is slow and most likely a waste of resources. Always prefer indexed rendering!
    pub fn de_index_all_buffers(&mut self, g: &mut Context) {
        // ImVector<ImDrawVert> new_vtx_buffer;
        let mut new_vtx_buffer: Vec<DrawVertex> = vec![];
        self.total_vtx_count = 0;
        self.total_idx_count = 0;
        // for (int i = 0; i < cmd_lists_count; i += 1)
        for i in 0..self.cmd_lists_count {
            // ImDrawList* cmd_list = CmdLists[i];
            let cmd_list_id = self.cmd_lists[i];
            let cmd_list = g.draw_list_mut(cmd_list_id);
            if cmd_list.idx_buffer.is_empty() {
                continue;
            }
            new_vtx_buffer.resize(cmd_list.idx_buffer.len(), DrawVertex::default());
            // for (int j = 0; j < cmd_list.idx_buffer.size; j += 1)
            for j in 0..cmd_list.idx_buffer.len() {
                new_vtx_buffer[j] = cmd_list.vtx_buffer[cmd_list.idx_buffer[j]];
            }
            // cmd_list.vtx_buffer.swap(new_vtx_buffer);
            cmd_list.vtx_buffer = new_vtx_buffer.to_owned();
            // cmd_list.idx_buffer.resize(0);
            cmd_list.idx_buffer.clear();
            self.total_vtx_count += cmd_list.vtx_buffer.size;
    }
    }
    //      void  scale_clip_rects(const Vector2D& fb_scale); // Helper to scale the clip_rect field of each ImDrawCmd. Use if your final output buffer is at a different scale than Dear ImGui expects, or if there is a difference between your window resolution and framebuffer resolution.
    pub fn scale_clip_rects(&mut self, g: &mut Context, fb_scale: &Vector2D) {
        // for (int i = 0; i < cmd_lists_count; i += 1)
    for i in 0..self.cmd_lists.len()
        {
        // ImDrawList* cmd_list = CmdLists[i];
        let cmd_list_id = self.cmd_lists[i];
            let cmd_list = g.draw_list_mut(cmd_list_id);
            // for (int cmd_i = 0; cmd_i < cmd_list.cmd_buffer.size; cmd_i += 1)
            for cmd_i in 0 .. cmd_list.cmd_buffer.len()
        {
            // ImDrawCmd* cmd = &cmd_list.cmd_buffer[cmd_i];
            let cmd = &mut cmd_list.cmd_buffer[cmd_i];
            cmd.clip_rect = Vector4D::new(cmd.clip_rect.x * fb_scale.x, cmd.clip_rect.y * fb_scale.y, cmd.clip_rect.z * fb_scale.x, cmd.clip_rect.w * fb_scale.y);
        }
    }
    }
}

/// Pass this to your backend rendering function! valid after Render() and until the next call to NewFrame()
/// ImDrawData* ImGui::GetDrawData()
pub fn get_draw_data(g: &mut Context) -> Option<&mut DrawData>
{
    // ImGuiContext& g = *GImGui;
    // ImGuiViewportP* viewport = g.viewports[0];
    let viewport = &mut g.viewports[0];
    return if viewport.draw_data.valid { Some(&mut viewport.draw_data)} else { None}
}

/// static void AddWindowToDrawData(ImGuiWindow* window, int layer)
pub fn add_window_to_draw_data(g: &mut Context, window: &mut Window, layer: i32) {
    // ImGuiContext& g = *GImGui;
    // ImGuiViewportP* viewport = window.viewport;
    let viewport_id = window.viewport_id;
    let viewport = g.viewport_mut(viewport_id).unwrap();
    g.io.metrics_render_windows += 1;
    if window.flags.contains(&WindowFlags::DockNodeHost) {
        window.draw_list_id.channels_merge();
    }
    add_draw_list_to_draw_data(
        g,
        &mut viewport.draw_data_builder.layers[layer],
        window.draw_list_id,
    );
    // for (int i = 0; i < window.dc.ChildWindows.Size; i += 1)
    // {
    //     ImGuiWindow* child = window.dc.ChildWindows[i];
    //     if (IsWindowActiveAndVisible(child)) // Clipped children may have been marked not active
    //         AddWindowToDrawData(child, layer);
    // }
    for child_id in window.dc.child_windows.iter() {
        let win_obj = g.window_mut(*child_id).unwrap();
        if checks::is_window_active_and_visible(win_obj) {
            add_window_to_draw_data(g, win_obj, layer);
        }
    }
}

/// Layer is locked for the root window, however child windows may use a different viewport (e.g. extruding menu)
// static inline void AddRootWindowToDrawData(ImGuiWindow* window)
pub fn add_root_window_to_draw_data(g: &mut Context, window: &mut Window) {
    // AddWindowToDrawData(window, GetWindowDisplayLayer(window));
    add_window_to_draw_data(g, window, get::get_window_display_layer(window))
}


impl DrawDataBuilder {
    // void clear()                    { for (int n = 0; n < IM_ARRAYSIZE(Layers); n += 1) Layers[n].resize(0); }
    //     void ClearFreeMemory()          { for (int n = 0; n < IM_ARRAYSIZE(Layers); n += 1) Layers[n].clear(); }
    //     int  GetDrawListCount() const   { int count = 0; for (int n = 0; n < IM_ARRAYSIZE(Layers); n += 1) count += Layers[n].size; return count; }
    pub fn get_draw_list_count(&self) -> usize {
        self.layers[0].len() + self.layers[1].len()
    }
    //      void FlattenIntoSingleLayer();
    pub fn flatten_into_single_layer(&mut self) {
        // int n = Layers[0].Size;
        let mut n = self.layers[0].len();
        //     int size = n;
        let mut size = n;
        //     for (int i = 1; i < IM_ARRAYSIZE(Layers); i += 1)
        //         size += Layers[i].Size;
        for i in 1..self.layers.len() {
            size += self.layers[i].len();
        }
        //     Layers[0].resize(size);
        self.layers[0].reserve(size);
        //     for (int layer_n = 1; layer_n < IM_ARRAYSIZE(Layers); layer_n += 1)
        for layer_n in 1..self.layers.len() {
            //     {
            //         ImVector<ImDrawList*>& layer = Layers[layer_n];
            let layer = &mut self.layers[layer_n];
            //         if (layer.empty())
            if layer.is_empty() {
                continue;
            }
            //             continue;
            //         memcpy(&Layers[0][n], &layer[0], layer.Size * sizeof(ImDrawList*));
            self.layers[0][n] = self.layer[0];
            n += layer.len();
            layer.clear();
            //         n += layer.Size;
            //         layer.resize(0);
            //     }
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct DrawDataBuilder {
    // ImVector<ImDrawList*>   Layers[2];           // Global layers for: regular, tooltip
    pub layers: [Vec<Id32>; 2],
}
