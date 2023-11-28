use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(fmt::Debug, Hash, PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
pub struct UserId(pub u64);

#[derive(fmt::Debug, Hash, PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
pub struct EntityId(pub u64);
