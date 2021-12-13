use std::sync::Arc;

use amazon_isr::api::amazon::fetch_asins_from_keyword;
use reqwest::{cookie::Jar, Url};

#[tokio::main]
async fn main() {
    //
    let client = reqwest::Client::builder()
        .cookie_store(true)
        .cookie_provider({
            let jar = reqwest::cookie::Jar::default();
            let amazon_url = Url::parse("https://www.amazon.com/").unwrap();
            jar.add_cookie_str("csm-hit=tb:s-3CY0B7P4R0SMT2058YTE|1639105423279&t:1639105423279&adb:adblk_no", &amazon_url);
            jar.add_cookie_str("i18n-prefs=USD", &amazon_url);
            jar.add_cookie_str("session-id=132-2091931-4217730", &amazon_url);
            jar.add_cookie_str("session-id-time=2082787201l", &amazon_url);
            jar.add_cookie_str("session-token=IPO+6Zh1p7hjNVG9TtGc/PKYDftpjUDg5YisKceRSQPyvmEr80T1w1Al7YpyqAFkbIjkbnNRSw+ceRQQOrOK3BJRx47/Kh4xfpbVvO7K0iDVCIETYmjdgX307h8xwpUVLOff78UghsN6n1916WmmjY33C+kMpUCS3+oAkW7jT0eQb2pVfXmSXXISGNgdt71g", &amazon_url);
            jar.add_cookie_str("skin=noskin", &amazon_url);
            jar.add_cookie_str("ubid-main=134-6526928-0717007", &amazon_url);
            Arc::new(jar)
        })
        .build()
        .unwrap();

    let results = fetch_asins_from_keyword(&client, "toaster oven")
        .await
        .unwrap();

    dbg!(results);
}
