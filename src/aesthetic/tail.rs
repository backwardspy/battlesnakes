use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Tail {
    Default,
    Comet,
}

impl Default for Tail {
    fn default() -> Self {
        Self::Default
    }
}
