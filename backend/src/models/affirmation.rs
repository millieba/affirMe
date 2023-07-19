#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]

pub struct Affirmation {
    pub text: String,
    pub tags: Vec<String>,
}
