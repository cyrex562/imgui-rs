// typedef struct
// {
//    c_uchar x;
// } stbrp_node;

use libc::c_uchar;

#[derive(Default,Debug,Copy, Clone)]
pub struct stbrp_node {
    pub x: c_uchar
}
