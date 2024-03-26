use std::future::Future;

pub fn block_on<F, T>(future: F) -> T
where
    F: Future<Output = T>,
{
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("failed to build tokio runtime")
        .block_on(future)
}
