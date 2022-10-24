use std::collections::HashMap;
use std::collections::hash_map::Iter;


pub struct RequestParams {
    params: HashMap<String, String>,
}

impl RequestParams {
    pub fn from_request_url(url: &str) -> RequestParams {
        let mut params = HashMap::new();

        match url.contains('?') {
            true => url.split_once('?').unwrap().1,
            false => url,
        }
        .split('&')
		// map to (Option, item) by splitting by '='. If Option is None, we use item as the key,
		// to map options like these: '?foo&bar'
        .map(|item| (item.split_once('='), item))
        .for_each(|pair_or_item| {
            if let Some((key, value)) = pair_or_item.0 {
                params.insert(String::from(key), String::from(value));
            } else {
				params.insert(String::from(pair_or_item.1), String::from(""));
			}
        });

        RequestParams { params }
    }

	pub fn pairs(&self) -> Iter<String,String> {
		self.params.iter()
	}

	pub fn get<'a>(&'a self, key: &str, default: &'a str) -> &'a str {
		match self.params.get(key) {
			Some(value) => value,
			None => default
		}
	}

	pub fn get_i64(&self, key: &str) -> Option<i64>  {
		match self.params.contains_key(key) {
			true => match str::parse::<i64>(self.get(key, "")) {
				Ok(nr) => Some(nr),
				Err(_) => None
			},
			false => None
		}
	}
}
