use std::collections::HashMap;

#[derive(Debug)]
pub struct HeaderMap {
	header_map: HashMap<String, String>
}

impl HeaderMap {
	pub fn builder(header_lines: &Vec<String>) -> HeaderMap {
        let mut header_map = HeaderMap {
			header_map: HashMap::new()
		};

        for line in header_lines {
            let (left, right) = match line.split_once(':') {
                Some(res) => (res.0.trim(), res.1.trim()),
                None => continue,
            };
			header_map.header_map.insert(left.to_lowercase(), String::from(right));
        }
        header_map
	}

	pub fn get(&self, key: &str) -> Option<String> {
		match self.header_map.get(&key.to_lowercase()) {
			Some(val) => Some(String::from(val)),
			None => None
		}
	}
}
