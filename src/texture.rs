use crate::types::Id32;

// ImTexture: user data for renderer backend to identify a texture [Compile-time configurable type]
// - To use something else than an opaque void* pointer: override with e.g. '#define ImTextureID MyTextureType*' in your imconfig.h file.
// - This can be whatever to you want it to be! read the FAQ about ImTextureID for details.
// #ifndef ImTextureID
// typedef void* ImTextureID;          // Default: store a pointer or an integer fitting in a pointer (most renderer backends are ok with that)
pub type TextureId = Id32;
