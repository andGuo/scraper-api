use crate::model::Page;
use serde::Serialize;

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[derive(Serialize, Debug)]
pub struct PageResponse {
    pub id: String,
    pub url: String,
    pub created_at: String,
    pub updated_at: String,
    pub title: String,
    pub keywords: Vec<String>,
    pub text_content: String,
    pub out_links: Vec<String>,
    pub in_links: Vec<String>,
    pub page_rank: f64,
}

impl From<Page> for PageResponse {
    fn from(page: Page) -> Self {
        PageResponse {
            id: page.id.to_hex(),
            url: page.url,
            created_at: page.created_at.to_rfc3339(),
            updated_at: page.updated_at.to_rfc3339(),
            title: page.title,
            keywords: page.keywords,
            text_content: page.text_content,
            out_links: page.out_links,
            in_links: page.in_links,
            page_rank: page.page_rank,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct PagesResponse {
    pub status: &'static str,
    pub data: Vec<PageResponse>,
}
