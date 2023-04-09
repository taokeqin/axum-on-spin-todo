use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct Todo {
    pub id: Uuid,
    pub text: String,
    pub completed: bool,
}
