use crate::orig_imgui_single_file;
use crate::imgui_text_range::ImGuiTextRange;

/// Helper: Parse and apply text filters. In format "aaaaa[,bbbb][,ccccc]"
#[derive(Default,Debug,Clone)]
pub struct ImGuiTextFilter
{
    pub InputBuf: String,
    pub Filters: Vec<ImGuiTextRange>,
    pub CountGrep: i32,

    // [Internal]

    // char                    InputBuf[256];
    // ImVector<ImGuiTextRange>Filters;
    // int                     CountGrep;
}

impl ImGuiTextFilter {
    //            ImGuiTextFilter(const char* default_filter = "");
    pub fn new(default_filter: &String) -> Self {
        let mut out = Self {
            ..Default()
        };
        out.InputBuf[0] = 0;
        out.countGrep = 0;
        if default_filter
        {
            out.InputBuf = default_filter.clone();
            out.Build();
        }
        out
    }
    //  bool      Draw(const char* label = "Filter (inc,-exc)", float width = 0.0);  // Helper calling InputText+build
    pub fn Draw(&mut self, label: &String, width: f32) -> bool {
        if width != 0.0 {
            orig_imgui_single_file::SetNextItemWidth(width);
        }
        let value_changed = InputText(label, self.InputBuf, IM_ARRAYSIZE(self.InputBuf));
        if (value_changed) {
            self.Build();
        }
        return value_changed;
    }
    //  bool      PassFilter(const char* text, const char* text_end = NULL) const;
    pub fn PassFilter(&mut self, text: &mut String, text_end: &String) -> bool {

        if self.Filters.empty() {
            return true;
        }

    if (text.is_empty()) {
        *text = String::from("");
    }

    // for (int i = 0; i != Filters.size; i += 1)
    for i in 0 .. self.Filters.size
        {
        // const ImGuiTextRange& f = Filters[i];
        let f = self.Filters[i];
            if (f.empty()){
            continue;}
        if (f.b[0] == '-')
        {
            // Subtract
            if (ImStristr(text, text_end, f.b + 1, f.e) != NULL) {
                return false;
            }
        }
        else
        {
            // Grep
            if (ImStristr(text, text_end, f.b, f.e) != NULL) {
                return true;
            }
        }
    }

    // Implicit * grep
    if (CountGrep == 0) {
        return true;
    }

    return false;


    }
    //  void      build();
    pub fn Build(&mut self) {
        // Filters.resize(0);
        self.Filters.reserve(0);
        // ImGuiTextRange input_range(InputBuf, InputBuf + strlen(InputBuf));
        let mut input_range = ImGuiTextRange::new(&mut self.InputBuf, &mut self.InputBuf + self.InputBuf.len());
        // input_range.split(',', &Filters);
        text_filter_text_range_split(',', &mut self.Filters);

        self.countGrep = 0;
        // for (int i = 0; i != Filters.size; i += 1)
        for i in 0..self.Filters.size {
            let f = Filters[i];
            while (f.b < f.e && ImCharIsBlankA(f.b[0])) {
                f.b += 1;
            }
            while (f.e > f.b && ImCharIsBlankA(f.e[-1])) {
                f.e -= 1;
            }
            if (f.empty()) {
                continue;
            }
            if (Filters[i].b[0] != '-') {
                CountGrep += 1;
            }
        }
    }
    // void                clear()          { InputBuf[0] = 0; build(); }
    pub fn Clear(&mut self) {
        self.InputBuf.clear();
        self.Filters.clear();
        self.countGrep = 0;
    }
    // bool                IsActive() const { return !Filters.empty(); }
    pub fn IsActive(&self) -> bool {
        !self.Filters.is_empty()
    }



//
// bool ImGuiTextFilter::PassFilter(const char* text, const char* text_end) const
// {
//
// }

}


// void ImGuiTextFilter::ImGuiTextRange::split(char separator, ImVector<ImGuiTextRange>* out) const
pub fn text_filter_text_range_split(separator: char, out: *mut Vec<ImGuiTextRange>)
{
    // out->resize(0);

    // const char* wb = b;
    let wb = b;
    // const char* we = wb;

    while (we < e)
    {
        if (*we == separator)
        {
            out.push_back(ImGuiTextRange(wb, we));
            wb = we + 1;
        }
        we += 1;
    }
    if (wb != we)
        out.push_back(ImGuiTextRange(wb, we));
}
