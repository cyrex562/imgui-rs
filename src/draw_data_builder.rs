use crate::draw_list::DrawList;

impl DrawDataBuilder {
    // void clear()                    { for (int n = 0; n < IM_ARRAYSIZE(Layers); n += 1) Layers[n].resize(0); }
    //     void ClearFreeMemory()          { for (int n = 0; n < IM_ARRAYSIZE(Layers); n += 1) Layers[n].clear(); }
    //     int  GetDrawListCount() const   { int count = 0; for (int n = 0; n < IM_ARRAYSIZE(Layers); n += 1) count += Layers[n].size; return count; }
    pub fn get_draw_list_count(&self) -> usize {
        self.layers[0].len() + self.layers[1].len()
    }
    //      void FlattenIntoSingleLayer();
    pub fn flatten_into_single_layer(&mut self) {
        todo!()
    }
}

#[derive(Debug,Clone,Default)]
pub struct DrawDataBuilder
{
    // ImVector<ImDrawList*>   Layers[2];           // Global layers for: regular, tooltip
    pub layers: [Vec<DrawList>; 2],
}
