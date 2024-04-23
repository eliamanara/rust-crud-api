#[derive(Serialize, Deserialize)]
pub struct Book {
    pub id: Option<i32>,
    pub author: String,
    pub title: String,
}
