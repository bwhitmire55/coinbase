pub struct Param {
    pub key: String,
    pub value: String,
}

pub struct Url {
    url: String,
}

impl Url {
    pub fn new(_url: &str) -> Url {
        Url { 
            url: _url.to_string(),
        }
    }

    pub fn get(&self) -> &str {
        &self.url
    }

    pub fn add_param(mut self, key: &str, value: &str) -> Url {
        if self.url.find("?") == None {
            self.url.push_str(format!("?{}={}", key, value).as_str());
        } else {
            self.url.push_str(format!("&{}={}", key, value).as_str());
        }
        self
    }

    pub fn path(&self) -> String {
        if self.url.is_empty() {
            return String::new();
        }
        let matches: Vec<_> = self.url.match_indices("/").collect();
        println!("Path: |{}|", String::from_utf8_lossy(&self.url.as_bytes()[matches[2].0..])[..].to_string());
        String::from_utf8_lossy(&self.url.as_bytes()[matches[2].0..])[..].to_string()
    }

    pub fn fill_data(&mut self, data: &str) {
        self.url = self.url.replace("{}", &data);
    }
}