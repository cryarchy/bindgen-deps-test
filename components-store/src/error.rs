#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to get a read lock on engine")]
    ModulesReadLock,
    #[error("failed to get a write lock on engine")]
    ModulesWriteLock,
    #[error("module not found: {0}")]
    ModuleNotFound(String),
}
