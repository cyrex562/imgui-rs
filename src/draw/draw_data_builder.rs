use crate::draw::draw_list::DrawList;
use crate::types::Id32;

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
