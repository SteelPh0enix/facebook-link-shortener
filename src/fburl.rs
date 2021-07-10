use log::{debug, trace};
extern crate pretty_env_logger;

const FBID_PATTERNS: [&str; 5] = [
  "&__cft__",
  "?__cft__",
  "&__eep__",
  "?__eep__",
  "&hoisted_section_header_type=recently_seen",
];
const FBURL_PATTERNS: [&str; 3] = ["facebook.", "fb.", "fbcdn."];

pub fn has_fbid(url: &str) -> bool {
  debug!("Checking if {} has facebook junk", url);
  for pattern in FBID_PATTERNS {
    trace!("Looking for {}...", pattern);
    if url.find(pattern).is_some() {
      debug!("Found!");
      return true;
    }
  }
  debug!("No patterns found!");
  false
}

pub fn is_facebook_url(potential_url: &str) -> bool {
  for url in FBURL_PATTERNS {
    if potential_url.find(url).is_some() {
      debug!("{} is a facebook URL", potential_url);
      return true;
    }
  }
  debug!("{} is NOT a facebook URL", potential_url);
  false
}

pub fn is_dirty_facebook_url(potential_url: &str) -> bool {
  let check_result = is_facebook_url(potential_url) && has_fbid(potential_url);
  if check_result {
    debug!("{} is a dirty facebook URL", potential_url);
  } else {
    debug!("{} is NOT a dirty facebook URL", potential_url);
  }
  check_result
}

pub fn shorten_facebook_url(url: &str) -> String {
  let mut clean_url = String::from(url);

  debug!("Shortening {}", url);

  for pattern in FBID_PATTERNS {
    if let Some(index) = clean_url.find(pattern) {
      trace!("Found patter {} at index {}", pattern, index);
      clean_url = clean_url.split_at(index).0.to_string();
    }
  }

  debug!("Cleaned URL: {}", clean_url);

  clean_url
}

#[cfg(test)]
mod test {
  use std::sync::Once;

  static INIT: Once = Once::new();

  pub fn initialize() {
    INIT.call_once(|| {
      pretty_env_logger::init_timed();
    });
  }

  const DIRTY_FACEBOOK_URLS: [&str; 4] = [
      "https://www.facebook.com/groups/414809682046969/?multi_permalinks=1652888561572402&hoisted_section_header_type=recently_seen&__cft__[0]=AZUYuomAKQLJuTXiyu04PLTgH30eJQ2usyHLrq74TzO9XF8BBtfweJGup9lQ7kV51nY_Dyv1dqHzyG4DnDiAv21i-5k_sp-KgCgJvOtonwMPhmchdhJKa6jURm2rfoZhXpTGK-KyeniBS6iAYMDg-vtMkftorZsn-bz62_u4nrump70WtFR1n6KCmA5hcYO2Ik8&__tn__=%2CO%2CP-R",
      "https://www.facebook.com/ads/about/?__cft__[0]=AZVKQxk0uHIoJBaTPnS-WyOCBR3kQHaenuHmD9zRffhOWwmH3p22dlC_x8Zus0Mezm3KDA7Gmaidwj2EbuLhpBdprghop5Q6ttFdoy8VihZSPv6JXJbtvEmeaCRPwYX1OpkWvlPnE6YSHlNEWdjYGuyZZpxtdKjGW7YatiEk7b8Q3X7BdN0ADcfC-2wOdTNxIvQlPt_oAErTiHGeJhcqHQxAomXnkWdT_RqAdRZ8Z9GnWkLRzD47KAs_9oL9fF2qQIsDgKpycheUsiLvM65LnkKk-EEcADNksDBZ-g-by5ZNIA&__tn__=%2CP-R",
      "https://www.facebook.com/taniabaniapl/posts/4403885436302581?__cft__[0]=AZXaxWQTaxH3VjEs-0pz95y6QgEezvMSh9TklrumI1rTKzbvaGHd_5UNAtlah6r185XGehqw2XGsQhMIzzu7VXBfllu6UI07PSs88qWCx-X2cUK6qF2f475to79gb6D7VhQbuo16jj4cX8Th1_wtE58JxRNB2DTipWCL-GTU2GK6M-RBXw1eNGjyJWauYFju41Y&__tn__=%2CO%2CP-R",
      "https://www.facebook.com/hashtag/monitor?__eep__=6&__cft__[0]=AZXltGNClTyhjkJUhkY7HsqhFdsfHWTsvGcSG8Flxp7G3VZBolOk8ccnsKx2z6f10PInBExGLqMsvfzKxBdLfiE1hjsx_SE_67-879CnRn3NvMMmA125yKE_CXSHfiYAS4MW6eE-c7PTe3dOsdKwmDaWjLMdyQ0ywWAMF3G4zfAmlv27h1MKcaTRNlHGqz46qLA&__tn__=*NK-R",
    ];

  const CLEANED_FACEBOOK_URLS: [&str; 4] = [
    "https://www.facebook.com/groups/414809682046969/?multi_permalinks=1652888561572402",
    "https://www.facebook.com/ads/about/",
    "https://www.facebook.com/taniabaniapl/posts/4403885436302581",
    "https://www.facebook.com/hashtag/monitor",
  ];

  const CLEAN_FACEBOOK_URLS: [&str; 9] = [
      "www.facebook.com",
      "https://www.facebook.com/",
      "http://www.facebook.com/",
      "https://www.facebook.com/groups/414809682046969/?multi_permalinks=1652888561572402",
      "https://scontent-waw1-1.xx.fbcdn.net/v/t1.6435-9/215055290_1380888338977570_1122198564338892848_n.jpg?_nc_cat=106&_nc_rgb565=1&ccb=1-3&_nc_sid=825194&_nc_ohc=b98dVh24eLEAX8olIs0&_nc_ht=scontent-waw1-1.xx&oh=71b3301e635d6bff5db7b004eb4a648e&oe=60EB1372",
      "https://www.facebook.com/ads/about/",
      "https://www.facebook.com/taniabaniapl/posts/4403885436302581",
      "https://www.facebook.com/hashtag/monitor",
      "https://www.facebook.com/groups/2596603887106122/"
    ];

  #[test]
  fn check_if_is_facebook_url() {
    initialize();

    for url in CLEAN_FACEBOOK_URLS {
      println!("Checking if {} IS a facebook URL", url);
      assert_eq!(super::is_facebook_url(url), true);
    }

    for url in DIRTY_FACEBOOK_URLS {
      println!("Checking if {} IS a facebook URL", url);
      assert_eq!(super::is_facebook_url(url), true);
    }
  }

  #[test]
  fn check_if_has_fbid() {
    initialize();

    for url in CLEAN_FACEBOOK_URLS {
      println!("Checking if {} DOES NOT HAVE facebook junk", url);
      assert_eq!(super::has_fbid(url), false);
    }

    for url in DIRTY_FACEBOOK_URLS {
      println!("Checking if {} DOES HAVE facebook junk", url);
      assert_eq!(super::has_fbid(url), true);
    }
  }

  #[test]
  fn check_if_is_long_facebook_url() {
    initialize();

    for url in DIRTY_FACEBOOK_URLS {
      println!("Checking if {} IS a long facebook URL", url);
      assert_eq!(super::is_dirty_facebook_url(url), true);
    }

    for url in CLEAN_FACEBOOK_URLS {
      println!("Checking if {} IS NOT a long facebook URL", url);
      assert_eq!(super::is_dirty_facebook_url(url), false);
    }
  }

  #[test]
  fn check_link_shortening() {
    initialize();

    for (dirty_url, clean_url) in DIRTY_FACEBOOK_URLS.iter().zip(CLEANED_FACEBOOK_URLS.iter()) {
      println!("Shortening\n{}\nto\n{}", dirty_url, clean_url);
      assert_eq!(super::shorten_facebook_url(dirty_url).eq(clean_url), true);
    }
  }
}
