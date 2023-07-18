#[derive(Debug, serde::Deserialize, serde::Serialize)]

pub struct Affirmation {
    pub text: String,
    pub tags: Vec<String>,
}
