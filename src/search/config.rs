use std::io;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
	pub genius_search_endpoint: String,
	pub genius_api_key: String,
	pub youtube_api_key: String,
}

impl ::std::default::Default for Config {
	fn default() -> Self {
		Self {
			genius_search_endpoint: "api.genius.com/search".to_string(),
			genius_api_key: String::new(),
			youtube_api_key: String::new(),
		}
	}
}

pub fn configure(cfg: Config) -> Config {
	let mut new_genius_key = String::new();
	let mut new_youtube_key: String = String::new();

	println!("Enter your Genius Client Access Token: [{}]", cfg.genius_api_key);
	io::stdin().read_line(&mut new_genius_key).expect("Error reading Genius Token");

	println!("Enter your YouTube API key: [{}]", cfg.youtube_api_key);
	io::stdin().read_line(&mut new_youtube_key).expect("Error reading YouTube Key");

	Config {
		genius_search_endpoint: cfg.genius_search_endpoint,
		genius_api_key: if new_genius_key.trim() == "" { 
			cfg.genius_api_key 
		} else { 
			new_genius_key.trim().to_string() 
		},
		youtube_api_key: if new_youtube_key.trim() == "" { 
			cfg.youtube_api_key 
		} else { 
			new_youtube_key.trim().to_string() 
		},
	}
}