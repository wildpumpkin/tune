extern crate serde;
use std::collections::HashMap;
use std::error::Error;
use crate::http::Query;
use crate::config::Config;
use serde::{Serialize, Deserialize};
use serde_json;
pub struct Request<T> {
	pub title: T,
	pub key: T,
}

impl Query for Request<&str> {
	fn get_params(&self) -> HashMap<String, String> {
		let mut map = HashMap::new();
		map.insert("part".to_string(), "snippet".to_string());
		map.insert("type".to_owned(), "video".to_owned());
		map.insert("q".to_string(), self.title.to_string());
		map.insert("key".to_string(), self.key.to_string());

		map
	}
}

pub struct YouTubeClient {
	video_search_endpoint: String,
}

impl YouTubeClient {
	pub fn with_config(cfg: Config) -> Self {
		Self {
			video_search_endpoint: cfg.youtube_search_endpoint,
		}
	}

	pub async fn search(&self, req: Request<&str>) -> Result<Response, Box<dyn Error>>{
		let resp = reqwest::get(
			&req.get_url(self.video_search_endpoint.clone()).to_owned()
		).await?;
		let results: Response = serde_json::from_str(
			&resp.text().await?.to_owned()
		)?;

			Ok(results)
	}
}
#[derive(Serialize, Deserialize)]
pub struct Response {
	pub items: Vec<Video>,
}

#[derive(Serialize, Deserialize)]
pub struct Video {
	id: VideoId,
	snippet: VideoInfo,
}

impl Video {
	pub fn get_title(&self) -> String {
		format!("{}\n{}", self.snippet.title, self.snippet.description)
	}

	pub fn get_location(&self) -> String {
		format!("https://www.youtube.com/watch?v={}", self.id.video_id)
	}
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct VideoId {
	video_id: String,
}

#[derive(Serialize, Deserialize)]
struct VideoInfo {
	title: String,
	description: String,
}