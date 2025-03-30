// magazines.rs
use crate::{common::Pagination, JikanClient, JikanError};
use serde::{Deserialize, Serialize};

pub enum OrderBy {
    None,
    MalId,
    Name,
    Count,
}

pub enum Sort {
    None,
    Asc,
    Desc,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MagazineResponse<T> {
    pub data: T,
    pub pagination: Option<Pagination>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Magazine {
    pub mal_id: i32,
    pub name: String,
    pub url: String,
    pub count: i32,
}

impl JikanClient {
    pub async fn get_magazines(
        &self,
        page: Option<u32>,
        limit: Option<u32>,
        q: Option<String>,
        order_by: OrderBy,
        sort: Sort,
        letter: Option<String>,
    ) -> Result<MagazineResponse<Vec<Magazine>>, JikanError> {
        let mut params = Vec::new();

        if let Some(p) = page {
            params.push(format!("page={}", p));
        }

        if let Some(l) = limit {
            params.push(format!("limit={}", l));
        }

        if let Some(qr) = q {
            params.push(format!("q={}", qr));
        }

        let order = match order_by {
            OrderBy::None => String::new(),
            OrderBy::MalId => "order_by=mal_id".to_string(),
            OrderBy::Name => "order_by=name".to_string(),
            OrderBy::Count => "order_by=count".to_string(),
        };
        params.push(format!("{}", order));

        let so = match sort {
            Sort::None => String::new(),
            Sort::Asc => "sort=asc".to_string(),
            Sort::Desc => "sort=desc".to_string(),
        };
        params.push(format!("{}", so));

        if let Some(lt) = letter {
            params.push(format!("letter={}", lt));
        }

        let query = if !params.is_empty() {
            format!("?{}", params.join("&"))
        } else {
            String::new()
        };

        self.get(&format!("/magazines{}", query)).await
    }
}
