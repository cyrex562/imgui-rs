#[derive(Clone,Debug,Default)]
pub struct ImGuiTextRange
    {
        // const char*     b;
        // const char*     e;
        pub b: String,
        pub e: String,


    }

impl ImGuiTextRange {
    // ImGuiTextRange()                                { b = e = NULL; }
    pub fn new(b: &String, e: &String) -> Self {
        Self {
            b: b.clone(),
            e: e.clone()
        }
    }
    //     ImGuiTextRange(const char* _b, const char* _e)  { b = _b; e = _e; }
    //     bool            empty() const                   { return b == e; }
    pub fn empty(&self) -> bool {
        self.b == self.e
    }
    //      void  split(char separator, ImVector<ImGuiTextRange>* out) const;
    pub fn split(&mut self, separator: char) -> Vec<String> {
        todo!()
    }
}
