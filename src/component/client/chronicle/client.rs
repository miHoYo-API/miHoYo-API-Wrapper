//! A mode of Game
//!
//!

#[allow(unused)]
pub(crate) struct ChronicleCacheKey;


///
#[derive(Debug, Clone)]
pub(crate) struct Chronicle<T>(pub(crate) T)
where T: Send + Sync;

