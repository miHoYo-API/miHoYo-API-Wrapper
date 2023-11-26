pub(crate) mod genshin;
pub(crate) mod honkai;
pub(crate) mod starrail;



#[allow(unused)]
pub(crate) struct ChronicleCacheKey;


#[derive(Debug)]
pub(crate) struct ChronicleClient<T>(pub(crate) T)
where T: Send + Sync;