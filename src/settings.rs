use crate::types::Id32;

//-----------------------------------------------------------------------------
// [SECTION] Settings support
//-----------------------------------------------------------------------------
#[derive(Default,Debug,Clone)]
pub struct SettingsHandler
{
    // const char* TypeName;       // Short description stored in .ini file. Disallowed characters: '[' ']'
    pub type_name: String,
    // ImGuiID     TypeHash;       // == ImHashStr(TypeName)
    pub type_hash: Id32,
    // void        (*ClearAllFn)(ImGuiContext* ctx, ImGuiSettingsHandler* handler);                                // clear all settings data
    pub clear_all_fn: Option<fn(ctx: &mut DimgContext, handler: &mut SettingsHandler)>,
    // void        (*ReadInitFn)(ImGuiContext* ctx, ImGuiSettingsHandler* handler);                                // Read: Called before reading (in registration order)
    pub read_init_fn: Option<fn(ctx: &mut DimgContext, handler: &mut SettingsHandler)>,
    // void*       (*ReadOpenFn)(ImGuiContext* ctx, ImGuiSettingsHandler* handler, const char* name);              // Read: Called when entering into a new ini entry e.g. "[Window][name]"
    pub read_open_fn: Option<fn(ctx: &mut DimgContext, handler: &mut SettingsHandler, name: &String)>,
    // void        (*ReadLineFn)(ImGuiContext* ctx, ImGuiSettingsHandler* handler, void* entry, const char* line); // Read: Called for every line of text within an ini entry
    pub read_line_fn: Option<fn(ctx: &mut DimgContext, handler: &mut SettingsHandler, entry: &mut Vec<u8>, line: &String)>,
    // void        (*ApplyAllFn)(ImGuiContext* ctx, ImGuiSettingsHandler* handler);                                // Read: Called after reading (in registration order)
    pub apply_all_fn: Option<fn(ctx: &mut DimgContext, handler: &mut SettingsHandler)>,
    // void        (*WriteAllFn)(ImGuiContext* ctx, ImGuiSettingsHandler* handler, ImGuiTextBuffer* out_buf);      // Write: Output every entries into 'out_buf'
    pub write_all_fn: Option<fn(ctx: &mut DimgContext, handler: SettingsHandler, out_buf: &mut DimgTextBuffer)>,
    // void*       user_data;
    pub user_data: Vec<u8>,
    //ImGuiSettingsHandler() { memset(this, 0, sizeof(*this)); }
}
