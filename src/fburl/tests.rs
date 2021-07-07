#[cfg(test)]
mod test {
  use super::*;

  const facebook_urls: [&str; 9] = [
      "www.facebook.com",
      "https://www.facebook.com/",
      "http://www.facebook.com/",
      "https://www.facebook.com/groups/414809682046969/?multi_permalinks=1652888561572402&hoisted_section_header_type=recently_seen&__cft__[0]=AZUYuomAKQLJuTXiyu04PLTgH30eJQ2usyHLrq74TzO9XF8BBtfweJGup9lQ7kV51nY_Dyv1dqHzyG4DnDiAv21i-5k_sp-KgCgJvOtonwMPhmchdhJKa6jURm2rfoZhXpTGK-KyeniBS6iAYMDg-vtMkftorZsn-bz62_u4nrump70WtFR1n6KCmA5hcYO2Ik8&__tn__=%2CO%2CP-R",
      "https://scontent-waw1-1.xx.fbcdn.net/v/t1.6435-9/215055290_1380888338977570_1122198564338892848_n.jpg?_nc_cat=106&_nc_rgb565=1&ccb=1-3&_nc_sid=825194&_nc_ohc=b98dVh24eLEAX8olIs0&_nc_ht=scontent-waw1-1.xx&oh=71b3301e635d6bff5db7b004eb4a648e&oe=60EB1372",
      "https://www.facebook.com/ads/about/?__cft__[0]=AZVKQxk0uHIoJBaTPnS-WyOCBR3kQHaenuHmD9zRffhOWwmH3p22dlC_x8Zus0Mezm3KDA7Gmaidwj2EbuLhpBdprghop5Q6ttFdoy8VihZSPv6JXJbtvEmeaCRPwYX1OpkWvlPnE6YSHlNEWdjYGuyZZpxtdKjGW7YatiEk7b8Q3X7BdN0ADcfC-2wOdTNxIvQlPt_oAErTiHGeJhcqHQxAomXnkWdT_RqAdRZ8Z9GnWkLRzD47KAs_9oL9fF2qQIsDgKpycheUsiLvM65LnkKk-EEcADNksDBZ-g-by5ZNIA&__tn__=%2CP-R",
      "https://www.facebook.com/taniabaniapl/posts/4403885436302581?__cft__[0]=AZXaxWQTaxH3VjEs-0pz95y6QgEezvMSh9TklrumI1rTKzbvaGHd_5UNAtlah6r185XGehqw2XGsQhMIzzu7VXBfllu6UI07PSs88qWCx-X2cUK6qF2f475to79gb6D7VhQbuo16jj4cX8Th1_wtE58JxRNB2DTipWCL-GTU2GK6M-RBXw1eNGjyJWauYFju41Y&__tn__=%2CO%2CP-R",
      "https://www.facebook.com/hashtag/monitor?__eep__=6&__cft__[0]=AZXltGNClTyhjkJUhkY7HsqhFdsfHWTsvGcSG8Flxp7G3VZBolOk8ccnsKx2z6f10PInBExGLqMsvfzKxBdLfiE1hjsx_SE_67-879CnRn3NvMMmA125yKE_CXSHfiYAS4MW6eE-c7PTe3dOsdKwmDaWjLMdyQ0ywWAMF3G4zfAmlv27h1MKcaTRNlHGqz46qLA&__tn__=*NK-R",
      "https://www.facebook.com/groups/2596603887106122/"
    ];

  const cleaned_facebook_urls: [&str; 9] = [
      "www.facebook.com",
      "https://www.facebook.com/",
      "http://www.facebook.com/",
      "https://www.facebook.com/groups/414809682046969/?multi_permalinks=1652888561572402&hoisted_section_header_type=recently_seen",
      "https://scontent-waw1-1.xx.fbcdn.net/v/t1.6435-9/215055290_1380888338977570_1122198564338892848_n.jpg?_nc_cat=106&_nc_rgb565=1&ccb=1-3&_nc_sid=825194&_nc_ohc=b98dVh24eLEAX8olIs0&_nc_ht=scontent-waw1-1.xx&oh=71b3301e635d6bff5db7b004eb4a648e&oe=60EB1372",
      "https://www.facebook.com/ads/about/",
      "https://www.facebook.com/taniabaniapl/posts/4403885436302581",
      "https://www.facebook.com/hashtag/monitor",
      "https://www.facebook.com/groups/2596603887106122/"
    ];

  #[test]
  fn check_if_is_facebook_url() {
    for url in facebook_urls {
      assert!(fburl::is_facebook_url(url));
    }
  }
}