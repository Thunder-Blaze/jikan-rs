use crate::{JikanClient, JikanError};

pub enum GenreFilter {
    None,
    Genres,
    ExplicitGenres,
    Themes,
    Demographics,
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Genre {
    pub mal_id: u32,
    pub name: String,
    pub url: String,
    pub count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenreResponse {
    pub data: Vec<Genre>,
}

impl JikanClient {
    pub async fn get_anime_genres(&self, filter: GenreFilter) -> Result<GenreResponse, JikanError> {
        let query = match filter {
            GenreFilter::None => String::new(),
            GenreFilter::Genres => "?filter=genres".to_string(),
            GenreFilter::ExplicitGenres => "?filter=explicit_genres".to_string(),
            GenreFilter::Themes => "?filter=themes".to_string(),
            GenreFilter::Demographics => "?filter=demographics".to_string(),
        };

        self.get(&format!("/genres/anime{}", query)).await
    }

    pub async fn get_manga_genres(&self, filter: GenreFilter) -> Result<GenreResponse, JikanError> {
        let query = match filter {
            GenreFilter::None => String::new(),
            GenreFilter::Genres => "?filter=genres".to_string(),
            GenreFilter::ExplicitGenres => "?filter=explicit_genres".to_string(),
            GenreFilter::Themes => "?filter=themes".to_string(),
            GenreFilter::Demographics => "?filter=demographics".to_string(),
        };

        self.get(&format!("/genres/manga{}", query)).await
    }
}
