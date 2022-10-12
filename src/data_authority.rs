use libc::c_int;

// Use your programming IDE "Go to definition" facility on the names of the center columns to find the actual flags/enum lists.
// typedef int ImGuiDataAuthority;         // -> enum ImGuiDataAuthority_      // Enum: for storing the source authority (dock node vs window) of a field
pub type ImGuiDataAuthority = c_int;
