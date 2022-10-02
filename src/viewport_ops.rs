#![allow(non_snake_case)]

// static c_void SetupViewportDrawData(*mut ImGuiViewportP viewport, Vec<ImDrawList*>* draw_lists)
pub fn SetupViewportDrawData(viewport: *mut ImGuiViewport, draw_lists: *mut Vec<*mut ImDrawList>) {
    // When minimized, we report draw_data.DisplaySize as zero to be consistent with non-viewport mode,
    // and to allow applications/backends to easily skip rendering.
    // FIXME: Note that we however do NOT attempt to report "zero drawlist / vertices" into the ImDrawData structure.
    // This is because the work has been done already, and its wasted! We should fix that and add optimizations for
    // it earlier in the pipeline, rather than pretend to hide the data at the end of the pipeline.
    let is_minimized: bool = (viewport.Flags & ImGuiViewportFlags_Minimized) != 0;

    let io = GetIO();
    let mut draw_data = &mut viewport.DrawDataP;
    viewport.DrawData = draw_data; // Make publicly accessible
    draw_data.Valid = true;
    draw_data.CmdLists = if draw_lists.Size > 0 { draw_lists.Data } else { null_mut() };
    draw_data.CmdListsCount = draw_lists.Size;
    draw_data.TotalVtxCount = draw_data.TotalIdxCount = 0;
    draw_data.DisplayPos = viewport.Pos;
    draw_data.DisplaySize = if is_minimized { ImVec2::new2(0f32, 0f32) } else { viewport.Size };
    draw_data.FramebufferScale = io.DisplayFramebufferScale; // FIXME-VIEWPORT: This may vary on a per-monitor/viewport basis?
    draw_data.OwnerViewport = viewport;
    // for (let n: c_int = 0; n < draw_lists.Size; n++)
    for n in 0..draw_lists.len() {
        let mut draw_list: *mut ImDrawList = draw_lists.Data[n];
        draw_list._PopUnusedDrawCmd();
        draw_data.TotalVtxCount += draw_list.VtxBuffer.Size;
        draw_data.TotalIdxCount += draw_list.IdxBuffer.Size;
    }
}
