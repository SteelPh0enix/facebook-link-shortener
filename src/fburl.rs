mod tests;

const FBID_PATTERNS: [&str; 1] = ["?__cft__"];
const FBURL_PATTERNS: [&str; 3] = ["facebook.", "fb.", "fbcdn."];

pub fn has_fbid(url: &str) -> bool {
  for pattern in FBID_PATTERNS {
    if url.find(pattern).is_some() {
      return true;
    }
  }
  false
}

pub fn is_facebook_url(potential_url: &str) -> bool {
  for url in FBURL_PATTERNS {
    if potential_url.find(url).is_some() {
      return true;
    }
  }
  false
}

pub fn is_long_facebook_url(potential_url: &str) -> bool {
  is_facebook_url(potential_url) && has_fbid(potential_url)
}

pub fn shorten_facebook_url(url: &str) -> String {
  let mut clean_url = String::from(url);

  for pattern in FBID_PATTERNS {
    if let Some(index) = clean_url.find(pattern) {
      clean_url = clean_url.split_at(index).0.to_string();
    }
  }

  clean_url
}
