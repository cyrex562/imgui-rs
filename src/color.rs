use libc::c_int;

//   With Visual Assist installed: ALT+G ("VAssistX.GoToImplementation") can also follow symbols in comments.
// typedef int ImGuiCol;               // -> enum ImGuiCol_             // Enum: A color identifier for styling
pub type ImGuiCol = c_int;
