use reqwest::Error;
use serde::{Deserialize, Deserializer};
use serde::de::{self, Error as DeError};
use regex::Regex;

#[derive(Deserialize, Debug)]
pub struct Band {
    pub id: String, // ID is a string as per the API response
    pub name: String,
    pub country: String,
    pub location: Option<String>, // Optional field
    pub formedIn: Option<String>, // Optional field
    pub yearsActive: Option<String>, // Optional field
    pub genre: String,
    pub themes: Option<String>,
    pub label: Option<String>,
    pub bandCover: Option<String>,
    pub albums: Option<Vec<Album>>, 
}

#[derive(Deserialize, Debug)]
pub struct FullAlbum {
    pub id: String,
    pub title: String,
    pub band: Option<Band>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(deserialize_with = "clean_date")]
    pub date: Option<String>,
    pub link: String,
}

#[derive(Deserialize, Debug)]
pub struct Album {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub year: Option<String>,
    pub link: String,
}

#[derive(Debug, Deserialize)]
pub struct AlbumDetails {
    id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub album_type: String,
    pub releaseDate: String,
    pub catalogID: String,
    pub versionDescription: String,
    pub label: Option<String>,
    #[serde(rename = "format")]
    pub albumFormat: Option<String>,
    pub limitations: String,
    pub reviews: String,
    pub coverUrl: String,
    pub songs: Option<Vec<Song>>,
}

#[derive(Debug, Deserialize)]
pub struct Song {
    id: String,
    pub number: String,
    pub name: String,
    pub length: String,
    pub lyrics: String,
}


fn clean_date<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let date: Option<String> = Option::deserialize(deserializer)?;
    
    if let Some(date) = date {
        let re = Regex::new(r"<!--.*?-->").map_err(D::Error::custom)?;
        let cleaned_date = re.replace_all(&date, "").trim().to_string();
        Ok(Some(cleaned_date))
    } else {
        Ok(None)  // If the field is None, return None directly
    }
}

async fn fetch_data<T>(url: &str) -> Result<T, Error>
where
    T: serde::de::DeserializeOwned,
{
    let response = reqwest::get(url).await?;
    let data: T = response.json().await?;
    Ok(data)
}

// Search by band name
pub async fn search_by_band_name(query: &str) -> Result<Vec<Band>, Error> {
    let url = format!("https://metal-api.dev/search/bands/name/{}", query);
    fetch_data(&url).await
}

// Search by genre
pub async fn search_by_genre(query: &str) -> Result<Vec<Band>, Error> {
    let url = format!("https://metal-api.dev/search/bands/genre/{}", query);
    fetch_data(&url).await
}

// Search by album title
pub async fn search_by_album_title(query: &str) -> Result<Vec<FullAlbum>, Error> {
    let url = format!("https://metal-api.dev/search/albums/title/{}", query);
    fetch_data(&url).await
}

// Fetch band details
pub async fn get_band_details(band_id: &str) -> Result<Band, Error> {
    let url = format!("https://metal-api.dev/bands/{}", band_id);
    fetch_data(&url).await
}

pub async fn get_album_details(album_id: &str) -> Result<AlbumDetails, Error> {
    let url = format!("https://metal-api.dev/albums/{}", album_id);
    fetch_data(&url).await
}