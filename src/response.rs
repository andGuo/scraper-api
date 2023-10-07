use chrono::{DateTime, Utc};
use serde::Serialize;
use crate::model::Page;

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[derive(Serialize, Debug)]
struct PageResponse {
    pub id: String,
    pub url: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub title: String,
    pub keywords: Vec<String>,
    pub text_content: String,
    pub out_links: Vec<String>,
    pub in_links: Vec<String>,
    pub page_rank: Option<f64>,
}

#[derive(Serialize, Debug)]
pub struct PagesResponse {
    pub status: &'static str,
    pub data: Vec<Page>,
}