#[allow(unused)]
pub(crate) struct ChronicleCacheKey;



#[derive(Debug)]
pub(crate) struct Chronicle<T>(pub(crate) T)
where T: Send + Sync;

