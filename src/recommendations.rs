use crate::common::{Images, Pagination};
use crate::users::User;
use crate::{JikanClient, JikanError};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendationEntry {
    pub mal_id: i32,
    pub url: String,
    pub images: Images,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    pub mal_id: String,
    pub entry: Vec<RecommendationEntry>,
    pub content: String,
    pub user: User,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendationResponse {
    pub data: Vec<Recommendation>,
    pub pagination: Pagination,
}

impl JikanClient {
    pub async fn get_recent_anime_recommendations(
        &self,
        page: Option<u32>,
    ) -> Result<RecommendationResponse, JikanError> {
        let mut params = Vec::new();

        if let Some(p) = page {
            params.push(format!("page={}", p));
        }

        let query = if !params.is_empty() {
            format!("?{}", params.join("&"))
        } else {
            String::new()
        };

        self.get(&format!("/recommendations/anime{}", query)).await
    }

    pub async fn get_recent_manga_recommendations(
        &self,
        page: Option<u32>,
    ) -> Result<RecommendationResponse, JikanError> {
        let mut params = Vec::new();

        if let Some(p) = page {
            params.push(format!("page={}", p));
        }

        let query = if !params.is_empty() {
            format!("?{}", params.join("&"))
        } else {
            String::new()
        };

        self.get(&format!("/recommendations/manga{}", query)).await
    }
}
