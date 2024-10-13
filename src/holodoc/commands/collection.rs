extern crate reqwest;
use reqwest::header;
use serde::{Deserialize, Serialize};
extern crate regex;
use regex::Regex;
extern crate rand;
use rand::seq::SliceRandom;

static USER_AGENT: &str = "holodoc/0.1.3";
static BASE_URL: &str =
    "https://api.discogs.com/users/Ospreythirtyone/collection/folders/0/releases?page=1&per_page=75";

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CollectionReleasesResponse {
    pub pagination: Pagination,
    #[serde(default)]
    pub releases: Releases,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pagination {
    pub page: i64,
    pub pages: i64,
    pub per_page: i64,
    pub items: i64,
    pub urls: Urls,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Urls {
    #[serde(default)]
    pub last: String,
    #[serde(default)]
    pub next: String,
}

pub type Releases = Vec<Album>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Album {
    pub id: i64,
    pub instance_id: i64,
    pub date_added: String,
    pub rating: i64,
    pub basic_information: BasicInformation,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BasicInformation {
    pub id: i64,
    pub master_id: i64,
    pub master_url: Option<String>,
    pub resource_url: String,
    pub thumb: String,
    pub cover_image: String,
    pub title: String,
    pub year: i64,
    pub formats: Vec<Format>,
    pub artists: Vec<Artist>,
    pub labels: Vec<Label>,
    pub genres: Vec<String>,
    pub styles: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Format {
    pub name: String,
    pub qty: String,
    #[serde(default)]
    pub descriptions: Vec<String>,
    pub text: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Artist {
    pub name: String,
    pub anv: String,
    pub join: String,
    pub role: String,
    pub tracks: String,
    pub id: i64,
    pub resource_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Label {
    pub name: String,
    pub catno: String,
    pub entity_type: String,
    pub entity_type_name: String,
    pub id: i64,
    pub resource_url: String,
}

fn clean_artist_name(artist: String) -> Result<String, Box<dyn std::error::Error>> {
    let regex = Regex::new(r"\s*\(\d+\)\s*")?;
    let result = regex.replace_all(artist.as_str(), "");
    Ok(result.to_string())
}

fn format_release(release: Album) -> Result<String, Box<dyn std::error::Error>> {
    let title: String = release.basic_information.title;
    let artist: String = release
        .basic_information
        .artists
        .iter()
        .map(|artist: &Artist| clean_artist_name(artist.name.clone()))
        .collect::<Result<Vec<_>, _>>()?
        .join(" & ");

    Ok(format!("{} - {}", artist, title))
}

fn get_random_releases(releases: Vec<Album>) -> Vec<Album> {
    // let mut random_releases: Vec<Album> = Vec::new();

    let mut rng = &mut rand::thread_rng();
    let random_releases: Vec<Album> = releases
        .clone()
        .choose_multiple(&mut rng, 4)
        .cloned()
        .collect();

    random_releases
}

// Change the function signature to async
// Result type now uses async since we're working with futures.
pub async fn get_all_releases() -> Result<Vec<Album>, Box<dyn std::error::Error>> {
    // No change needed here, we can initialize headers synchronously.
    let mut headers = header::HeaderMap::new();
    let ua = USER_AGENT.parse()?;
    headers.insert("User-Agent", ua);

    // Use the asynchronous reqwest client instead of the blocking version.
    // `reqwest::Client` is the async version, so we change from `reqwest::blocking::Client`
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()?;

    let mut page = 1;
    let mut all_releases = Vec::new();

    // The loop stays the same, but the network request will now be async.
    loop {
        // Replace blocking `send()` and `text()` with their async equivalents.
        // The `await` keyword is required since these calls are async.
        let res: CollectionReleasesResponse = serde_json::from_str(
            &client
                .get(BASE_URL.replace("page=1&per_page=75", &format!("page={}&per_page=75", page)))
                .headers(headers.clone())
                .send() // Async send
                .await? // Await the send future
                .text() // Async text to read the body
                .await?, // Await the text future
        )?;

        // Collect the releases from the response
        all_releases.extend(res.releases);

        // Break the loop when all pages are fetched
        if page == res.pagination.pages {
            break;
        }

        // Move to the next page
        page += 1;
    }

    // Return the aggregated releases
    Ok(all_releases)
}

pub fn print_releases(releases: Vec<Album>) -> String {
    let mut output = String::new();

    let random_releases: Vec<Album> = get_random_releases(releases);

    for release in random_releases {
        let rel_or_err_str = match format_release(release) {
            Ok(fmtrel) => fmtrel,
            Err(e) => e.to_string(),
        };
        output.push_str(&format!("{}\n", rel_or_err_str));
    }
    output
}
