use serde::{Serialize, Deserialize};


#[derive (Debug, Serialize, Deserialize)]
pub struct Data {
    items: Vec<Card>,
}

#[derive (Debug, Serialize, Deserialize)]
pub struct Card{
    id: String, 
    // #[serde(rename = "collectionId")]
    // collection_id: String,
    // #[serde(rename = "collectionName")]
    // collection_name: String,
    // created: String,
    updated: String,
    question: String,
    answer: String,
    category: String,
    last_checked: i64,
}