use crate::types::Id32;

/// data payload for Drag and Drop operations: AcceptDragDropPayload(), GetDragDropPayload()
#[derive(Default,Debug,Clone)]
pub struct Payload
{
    // Members
    pub data: Vec<u8>,               // data (copied and owned by dear imgui)
    pub data_size: usize,         // data size

    // [Internal]
    pub source_id: Id32,         // Source item id
    pub source_parent_id: Id32,   // Source parent id (if available)
    pub data_frame_count: usize,   // data timestamp
    pub data_type: String, // char            data_type[32 + 1];   // data type tag (short user-supplied string, 32 characters max)
    pub preview: bool,            // Set when AcceptDragDropPayload() was called and mouse has been hovering the target item (nb: handle overlapping drag targets)
    pub delivery: bool,           // Set when AcceptDragDropPayload() was called and mouse button is released over the target item.


}

impl Payload {
    // ImGuiPayload()  { clear(); }
    pub fn new() -> Self {
        Self {
            data_frame_count: usize::MAX,
            preview: false,
            delivery: false,
            ..Default::default()
        }
    }
    // void clear()    { source_id = source_parent_id = 0; data = NULL; data_size = 0; memset(data_type, 0, sizeof(data_type)); data_frame_count = -1; preview = delivery = false; }
    pub fn clear(&mut self) {
        self.source_id = 0;
        self.source_parent_id = 0;
        self.data = vec![];
        self.data_size = 0;
        self.data_type = String::from("");
        self.data_frame_count = usize::MAX;
        self.preview = false;
        self.delivery = false;
    }

    // bool is_data_type(const char* type) const { return data_frame_count != -1 && strcmp(type, data_type) == 0; }
    pub fn is_data_type(&self, data_type: &String) -> bool {
        self.data_frame_count != usize::MAX && (*data_type == self.data_type)
    }

    // bool is_preview() const                  { return preview; }
    pub fn is_preview(&self) -> bool {
        self.preview
    }
    // bool is_delivery() const                 { return delivery; }
    pub fn is_delivery(&self) -> bool {
        self.delivery
    }
}
