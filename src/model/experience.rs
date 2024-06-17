use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, Deserialize, Serialize)]
pub struct Experience {
    pub level: u32,
    pub progress: f32,
    pub total: u32,
}
