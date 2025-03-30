use crate::common::{Images, Pagination};
use crate::{JikanClient, JikanError};

use serde::{Deserialize, Serialize};

pub enum AnimeType {
    None,
    Tv,
    Movie,
    Ova,
    Special,
    Ona,
    Music,
    Cm,
    Pv,
    TvSpecial,
}

pub enum MangaType {
    None,
    Manga,
    Novel,
    LightNovel,
    OneShot,
    Doujin,
    Manhwa,
    Manhua,
}

pub enum Filter {
    None,
    Airing,
    Upcoming,
    ByPopularity,
    Favorite,
}

pub enum MangaFilter {
    None,
    Publishing,
    Upcoming,
    ByPopularity,
    Favorite,
}

pub enum Rating {
    None,
    G,
    Pg,
    Pg13,
    R17,
    R,
    Rx,
}

pub enum ReviewType {
    None,
    Anime,
    Manga,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Title {
    pub r#type: String,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiredProp {
    pub day: Option<u32>,
    pub month: Option<u32>,
    pub year: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Aired {
    pub from: Option<String>,
    pub to: Option<String>,
    pub prop: AiredProp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Broadcast {
    pub day: Option<String>,
    pub time: Option<String>,
    pub timezone: Option<String>,
    pub string: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommonTemplate {
    pub mal_id: u32,
    pub r#type: Option<String>,
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trailer {
    pub youtube_id: Option<String>,
    pub url: Option<String>,
    pub embed_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Anime {
    pub mal_id: u32,
    pub url: String,
    pub images: Images,
    pub trailer: Option<Trailer>,
    pub approved: Option<bool>,
    pub titles: Option<Vec<Title>>,
    pub title: String,
    pub title_english: Option<String>,
    pub title_japanese: Option<String>,
    pub title_synonyms: Option<Vec<String>>,
    pub r#type: Option<String>,
    pub source: Option<String>,
    pub episodes: Option<u32>,
    pub status: Option<String>,
    pub airing: Option<bool>,
    pub aired: Option<Aired>,
    pub duration: Option<String>,
    pub rating: Option<String>,
    pub score: Option<f32>,
    pub scored_by: Option<u32>,
    pub rank: Option<u32>,
    pub popularity: Option<u32>,
    pub members: Option<u32>,
    pub favorites: Option<u32>,
    pub synopsis: Option<String>,
    pub background: Option<String>,
    pub season: Option<String>,
    pub year: Option<u32>,
    pub broadcast: Option<Broadcast>,
    pub producers: Option<Vec<CommonTemplate>>,
    pub licensors: Option<Vec<CommonTemplate>>,
    pub studios: Option<Vec<CommonTemplate>>,
    pub genres: Option<Vec<CommonTemplate>>,
    pub explicit_genres: Option<Vec<CommonTemplate>>,
    pub themes: Option<Vec<CommonTemplate>>,
    pub demographics: Option<Vec<CommonTemplate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manga {
    pub mal_id: u32,
    pub url: String,
    pub images: Images,
    pub approved: bool,
    pub titles: Vec<Title>,
    pub title: String,
    pub title_english: Option<String>,
    pub title_japanese: Option<String>,
    pub r#type: String,
    pub chapters: Option<u32>,
    pub volumes: Option<u32>,
    pub status: String,
    pub publishing: bool,
    pub published: Aired,
    pub score: Option<f32>,
    pub scored_by: Option<u32>,
    pub rank: Option<u32>,
    pub popularity: Option<u32>,
    pub members: Option<u32>,
    pub favorites: Option<u32>,
    pub synopsis: Option<String>,
    pub background: Option<String>,
    pub authors: Vec<CommonTemplate>,
    pub serializations: Vec<CommonTemplate>,
    pub genres: Vec<CommonTemplate>,
    pub explicit_genres: Vec<CommonTemplate>,
    pub themes: Vec<CommonTemplate>,
    pub demographics: Vec<CommonTemplate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Person {
    pub mal_id: u32,
    pub url: String,
    pub website_url: Option<String>,
    pub images: Images,
    pub name: String,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub alternate_names: Vec<String>,
    pub birthday: Option<String>,
    pub favorites: Option<u32>,
    pub about: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    pub mal_id: u32,
    pub url: String,
    pub images: Images,
    pub name: String,
    pub name_kanji: Option<String>,
    pub nicknames: Vec<String>,
    pub favorites: Option<u32>,
    pub about: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub url: String,
    pub images: Images,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reactions {
    pub overall: u32,
    pub nice: u32,
    pub love_it: u32,
    pub funny: u32,
    pub confusing: u32,
    pub informative: u32,
    pub well_written: u32,
    pub creative: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Review {
    pub mal_id: u32,
    pub url: String,
    pub r#type: String,
    pub reactions: Reactions,
    pub review: String,
    pub score: u32,
    pub entry: Anime,
    pub user: User,
    pub date: String,
    pub tags: Vec<String>,
    pub is_spoiler: bool,
    pub is_preliminary: bool,
    // pub episodes_watched: u32,    // I don't know it's exact type as it also has null value
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopResponse<T> {
    pub data: Vec<T>,
    pub pagination: Pagination,
}

impl JikanClient {
    pub async fn get_top_anime(
        &self,
        r#type: AnimeType,
        filter: Filter,
        rating: Rating,
        sfw: Option<bool>,
        page: Option<u32>,
        limit: Option<u32>,
    ) -> Result<TopResponse<Anime>, JikanError> {
        let mut params = Vec::new();

        let t_str = match r#type {
            AnimeType::None => String::new(),
            AnimeType::Tv => "tv".to_string(),
            AnimeType::Movie => "movie".to_string(),
            AnimeType::Ova => "ova".to_string(),
            AnimeType::Special => "special".to_string(),
            AnimeType::Ona => "ona".to_string(),
            AnimeType::Music => "music".to_string(),
            AnimeType::Cm => "cm".to_string(),
            AnimeType::Pv => "pv".to_string(),
            AnimeType::TvSpecial => "tv_special".to_string(),
        };
        if !t_str.is_empty() {
            params.push(format!("type={}", t_str));
        }

        let f_str = match filter {
            Filter::None => String::new(),
            Filter::Airing => "airing".to_string(),
            Filter::Upcoming => "upcoming".to_string(),
            Filter::ByPopularity => "bypopularity".to_string(),
            Filter::Favorite => "favorite".to_string(),
        };
        if !f_str.is_empty() {
            params.push(format!("filter={}", f_str));
        }

        let r_str = match rating {
            Rating::None => String::new(),
            Rating::G => "g".to_string(),
            Rating::Pg => "pg".to_string(),
            Rating::Pg13 => "pg13".to_string(),
            Rating::R17 => "r17".to_string(),
            Rating::R => "r".to_string(),
            Rating::Rx => "rx".to_string(),
        };
        if !r_str.is_empty() {
            params.push(format!("rating={}", r_str));
        }

        if let Some(s) = sfw {
            params.push(format!("sfw={}", s));
        }

        if let Some(p) = page {
            params.push(format!("page={}", p));
        }

        if let Some(l) = limit {
            params.push(format!("limit={}", l));
        }

        let query = if !params.is_empty() {
            format!("?{}", params.join("&"))
        } else {
            String::new()
        };

        self.get(&format!("/top/anime{}", query)).await
    }

    pub async fn get_top_manga(
        &self,
        r#type: MangaType,
        filter: MangaFilter,
        page: Option<u32>,
        limit: Option<u32>,
    ) -> Result<TopResponse<Manga>, JikanError> {
        let mut params = Vec::new();

        let t_str = match r#type {
            MangaType::None => String::new(),
            MangaType::Manga => "manga".to_string(),
            MangaType::Novel => "novel".to_string(),
            MangaType::LightNovel => "lightnovel".to_string(),
            MangaType::OneShot => "oneshot".to_string(),
            MangaType::Doujin => "doujin".to_string(),
            MangaType::Manhwa => "manhwa".to_string(),
            MangaType::Manhua => "manhua".to_string(),
        };
        if !t_str.is_empty() {
            params.push(format!("type={}", t_str));
        }

        let f_str = match filter {
            MangaFilter::None => String::new(),
            MangaFilter::Publishing => "publishing".to_string(),
            MangaFilter::Upcoming => "upcoming".to_string(),
            MangaFilter::ByPopularity => "bypopularity".to_string(),
            MangaFilter::Favorite => "favorite".to_string(),
        };
        if !f_str.is_empty() {
            params.push(format!("filter={}", f_str));
        }

        if let Some(p) = page {
            params.push(format!("page={}", p));
        }

        if let Some(l) = limit {
            params.push(format!("limit={}", l));
        }

        let query = if !params.is_empty() {
            format!("?{}", params.join("&"))
        } else {
            String::new()
        };

        self.get(&format!("/top/manga{}", query)).await
    }

    pub async fn get_top_people(
        &self,
        page: Option<u32>,
        limit: Option<u32>,
    ) -> Result<TopResponse<Person>, JikanError> {
        let mut params = Vec::new();
        if let Some(p) = page {
            params.push(format!("page={}", p));
        }
        if let Some(l) = limit {
            params.push(format!("limit={}", l));
        }
        let query = if !params.is_empty() {
            format!("?{}", params.join("&"))
        } else {
            String::new()
        };
        self.get(&format!("/top/people{}", query)).await
    }

    pub async fn get_top_characters(
        &self,
        page: Option<u32>,
        limit: Option<u32>,
    ) -> Result<TopResponse<Character>, JikanError> {
        let mut params = Vec::new();
        if let Some(p) = page {
            params.push(format!("page={}", p));
        }
        if let Some(l) = limit {
            params.push(format!("limit={}", l));
        }
        let query = if !params.is_empty() {
            format!("?{}", params.join("&"))
        } else {
            String::new()
        };
        self.get(&format!("/top/characters{}", query)).await
    }

    pub async fn get_top_reviews(
        &self,
        r#type: ReviewType,
        preliminary: Option<bool>,
        spoilers: Option<bool>,
        page: Option<u32>,
    ) -> Result<TopResponse<Review>, JikanError> {
        let mut params = Vec::new();
        let t_str = match r#type {
            ReviewType::None => String::new(),
            ReviewType::Anime => "anime".to_string(),
            ReviewType::Manga => "manga".to_string(),
        };
        if !t_str.is_empty() {
            params.push(format!("type={}", t_str));
        }
        if let Some(p) = preliminary {
            params.push(format!("preliminary={}", p));
        }
        if let Some(s) = spoilers {
            params.push(format!("spoilers={}", s));
        }
        if let Some(p) = page {
            params.push(format!("page={}", p));
        }
        let query = if !params.is_empty() {
            format!("?{}", params.join("&"))
        } else {
            String::new()
        };
        self.get(&format!("/top/reviews{}", query)).await
    }
}
