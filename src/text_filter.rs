use crate::orig_imgui_single_file;
use crate::imgui_text_range::ImGuiTextRange;
use crate::item::set_next_item_width;
use crate::text_range::TextRange;

/// Helper: Parse and apply text filters. In format "aaaaa[,bbbb][,ccccc]"
#[derive(Default,Debug,Clone)]
pub struct TextFilter
{
    pub input_buf: String,
    pub filters: Vec<TextRange>,
    pub count_grep: i32,
}

impl TextFilter {
    //            ImGuiTextFilter(const char* default_filter = "");
    pub fn new(default_filter: &String) -> Self {
        let mut out = Self {
            ..Default()
        };
        // out.input_buf[0] = 0;
        // out.countGrep = 0;
        if default_filter {
            out.input_buf = default_filter.clone();
            out.build();
        }
        out
    }

    //  bool      Draw(const char* label = "Filter (inc,-exc)", float width = 0.0);  // Helper calling InputText+build
    pub fn draw(&mut self, label: &String, width: f32) -> bool {
        if width != 0.0 {
           set_next_item_width(g, width);
        }
        let value_changed = input_text(label, &mut self.input_buf);
        if value_changed {
            self.Build();
        }
        return value_changed;
    }
    //  bool      PassFilter(const char* text, const char* text_end = None) const;
    pub fn pass_filter(&mut self, text: &mut String, text_end: &String) -> bool {
        if self.filters.empty() {
            return true;
        }

        if (text.is_empty()) {
            *text = String::from("");
        }

        // for (int i = 0; i != filters.size; i += 1)
        // for i in 0..self.filters.size
        for filter in self.filters.iter_mut()
        {
            // const ImGuiTextRange& f = filters[i];
            // let f = self.filters[i];
            if filter.empty() {
                continue;
            }
            if filter.b[0] == '-' {
                // Subtract
                // todo
                // if (ImStristr(text, text_end, f.b + 1, f.e) != None) {
                //     return false;
                // }
            } else {
                // Grep
                // todo
                // if (ImStristr(text, text_end, f.b, f.e) != None) {
                //     return true;
                // }
            }
        }

        // Implicit * grep
        if self.count_grep == 0 {
            return true;
        }

        return false;
    }

    //  void      build();
    pub fn build(&mut self) {
        // filters.resize(0);
        self.filters.reserve(0);
        // ImGuiTextRange input_range(input_buf, input_buf + strlen(input_buf));
        let mut input_range = TextRange::new(&mut self.input_buf, &mut self.input_buf + self.input_buf.len());
        // input_range.split(',', &filters);
        text_filter_text_range_split(',', &mut self.filters);

        self.count_grep = 0;
        // for (int i = 0; i != filters.size; i += 1)
        // for i in 0..self.filters.size {
        for f in self.filters.iter_mut() {
            // let f = self.filters[i];
            // while f.b < f.e && char_is_blank_a(f.b[0]) {
            //     f.b += 1;
            // }
            // TODO:
            while f.e > f.b && char_is_blank_a(f.e[-1]) {
                f.e -= 1;
            }
            if f.empty() {
                continue;
            }
            // if (self.filters.[i].b[0] != '-') {
            //     CountGrep += 1;
            // }
            if f.b[0] != '-' {
                self.count_grep += 1;
            }
        }
    }
    // void                clear()          { input_buf[0] = 0; build(); }
    pub fn clear(&mut self) {
        self.input_buf.clear();
        self.filters.clear();
        self.count_grep = 0;
    }
    // bool                IsActive() const { return !filters.empty(); }
    pub fn is_active(&self) -> bool {
        !self.filters.is_empty()
    }



//
// bool ImGuiTextFilter::PassFilter(const char* text, const char* text_end) const
// {
//
// }

}


// void ImGuiTextFilter::ImGuiTextRange::split(char separator, ImVector<ImGuiTextRange>* out) const
pub fn text_filter_text_range_split(separator: char, out: &mut Vec<TextRange>)
{
    // // out->resize(0);
    //
    // // const char* wb = b;
    // let wb = b;
    // // const char* we = wb;
    //
    // while we < e
    // {
    //     if (*we == separator)
    //     {
    //         out.push_back(ImGuiTextRange(wb, we));
    //         wb = we + 1;
    //     }
    //     we += 1;
    // }
    // if (wb != we)
    //     out.push_back(ImGuiTextRange(wb, we));
    todo!()
}
