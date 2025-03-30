// producers.rs
use crate::{
    character::*,
    common::{Images, Pagination},
    JikanClient, JikanError,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Producer {
    pub mal_id: i32,
    pub url: String,
    pub titles: Vec<TitleEntry>,
    pub images: Images,
    pub favorites: i32,
    pub count: i32,
    pub established: Option<String>,
    pub about: Option<String>,
    pub external: Option<Vec<ExternalEntry>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleEntry {
    #[serde(rename = "type")]
    pub type_: String,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalEntry {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProducerResponse<T> {
    pub data: T,
    pub pagination: Option<Pagination>,
}

pub enum FilterType {
    SortFilter,
    OrderFilter,
    None,
}

impl JikanClient {
    pub async fn get_producer_by_id(
        &self,
        id: i32,
    ) -> Result<ProducerResponse<Producer>, JikanError> {
        self.get(&format!("/producers/{}", id)).await
    }

    pub async fn get_producer_full_by_id(
        &self,
        id: i32,
    ) -> Result<ProducerResponse<Producer>, JikanError> {
        self.get(&format!("/producers/{}/full", id)).await
    }

    pub async fn get_producer_external(
        &self,
        id: i32,
    ) -> Result<ProducerResponse<Vec<ExternalEntry>>, JikanError> {
        self.get(&format!("/producers/{}/external", id)).await
    }

    pub async fn get_producer_search(
        &self,
        page: Option<i32>,
        limit: Option<i32>,
        query: Option<i32>,
        order_by: Option<OrderBy>,
        sort: Option<Sort>,
        letter: Option<String>,
    ) -> Result<ProducerResponse<Vec<Producer>>, JikanError> {
        let mut params: Vec<String> = Vec::new();

        if let Some(p) = page {
            params.push(format!("page={}", p));
        }
        if let Some(lim) = limit {
            params.push(format!("limit={}", lim));
        }
        if let Some(q) = query {
            params.push(format!("q={}", q));
        }
        if let Some(o) = order_by {
            let order = match o {
                OrderBy::MalId => "mal_id",
                OrderBy::Name => "name",
                OrderBy::Favorites => "favorites",
            };
            params.push(format!("order_by={}", order));
        }
        if let Some(s) = sort {
            let sort = match s {
                Sort::Asc => "asc",
                Sort::Desc => "desc",
            };
            params.push(format!("sort={}", sort));
        }

        if let Some(l) = letter {
            if l.len() != 1 {
                return Err(JikanError::BadRequest(
                    "Letter must be a single character".to_string(),
                ));
            }
            params.push(format!("letter={}", l));
        }
        let query = if !params.is_empty() {
            format!("?{}", params.join("&"))
        } else {
            String::new()
        };
        self.get(&format!("/producers{}", query)).await
    }

    pub async fn get_producers(&self) -> Result<ProducerResponse<Vec<Producer>>, JikanError> {
        self.get("/producers").await
    }
}
