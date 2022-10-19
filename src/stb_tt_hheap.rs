#![allow(non_camel_case_types)]

#[derive(Default,Debug,Copy, Clone)]
pub struct stbtt__hheap
{
   // struct stbtt__hheap_chunk *head;
   pub head: *mut stbtt__hheap_chunk,
   // c_void   *first_free;
   pub first_free: *mut c_void,
   // c_int    num_remaining_in_head_chunk;
    pub num_remaining_in_head_chunk: c_int
}
