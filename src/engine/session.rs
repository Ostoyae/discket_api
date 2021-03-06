use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::{RwLock, RwLockReadGuard};

use crate::engine::db::Db;
use crate::engine::EngineError;

pub trait NewSession {}

pub struct Session<T>(Arc<RwLock<T>>);

// #[async_trait]
// pub trait Engine {
//     async fn insert<T: ReqiredTraits>(&self, doc: T) -> Result<(), EngineError>;
// }

impl<T> Session<T> {
    pub fn from(t: T) -> Result<Session<T>, EngineError> {
        Ok(Session(Arc::new(RwLock::new(t))))
    }

    pub fn clone(&self) -> Arc<RwLock<T>> {
        self.0.clone()
    }

}

#[async_trait]
impl NewSession for Db {}

#[cfg(test)]
pub(crate) mod test {
    use crate::engine::db::{AuthType, Db};
    use crate::engine::EngineError;
    use crate::engine::session::Session;

    pub async fn common_session_db() -> Result<Session<Db>, EngineError> {
        let db = Db::new()
            .db_name("discket_dev")
            .auth_type(AuthType::Jwt {
                user: "discket",
                pass: "babyYoda",
            })
            .connect()
            .await?;
        let session: Session<Db> = Session::from(db)?;

        Ok(session)
    }
}
