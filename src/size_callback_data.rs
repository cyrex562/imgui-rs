use crate::vectors::two_d::Vector2D;

/// Resizing callback data to apply custom constraint. As enabled by SetNextWindowSizeConstraints(). Callback is called during the next Begin().
/// NB: For basic min/max size constraint on each axis you don't need to use the callback! The SetNextWindowSizeConstraints() parameters are enough.
#[derive(Default,Debug,Clone)]
pub struct SizeCallbackData
{
    // void*   user_data;       // Read-only.   What user passed to SetNextWindowSizeConstraints()
    pub user_data: Vec<u8>,
    // pub pos: Vector2D,            // Read-only.   window position, for reference.
    pub pos: Vector2D,
    // pub current_size: Vector2D,    // Read-only.   current window size.
    pub current_size: Vector2D,
    // pub desired_size: Vector2D,    // Read-write.  Desired size, based on user's mouse position. Write to this field to restrain resizing.
    pub desired_size: Vector2D,
}
