use amazon_isr::helium10::*;
use reqwest::{header::COOKIE, Client};

const WIP_HASH: Option<&str> = None;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut client = reqwest::Client::builder().build()?;

    /*
    Not sure how important these are....
    */

    // Log in to our account
    let accounts_res = client
        .get("https://members.helium10.com/api/v1/customers/accounts")
        .header(COOKIE, "signUpRequirementType=fe23686b4977cf81a53e69d67e7de24a9c6090131d666abb6c1babecf46de261a%3A2%3A%7Bi%3A0%3Bs%3A21%3A%22signUpRequirementType%22%3Bi%3A1%3Bs%3A20%3A%22phone_input_required%22%3B%7D; _gcl_au=1.1.2052847598.1632334553; _ga=GA1.3.1184175819.1632334553; current-marketplace=5a7deede86e62213f2adae2c21dde37c8919c5317898e27c859a9080214c380fa%3A2%3A%7Bi%3A0%3Bs%3A19%3A%22current-marketplace%22%3Bi%3A1%3Bs%3A13%3A%22ATVPDKIKX0DER%22%3B%7D; ab.storage.userId.4d3ca359-c724-43de-9b97-cb9f1fee4769=%7B%22g%22%3A%221544530874%22%2C%22c%22%3A1632334597512%2C%22l%22%3A1632334597512%7D; ab.storage.deviceId.4d3ca359-c724-43de-9b97-cb9f1fee4769=%7B%22g%22%3A%2260b39bb7-ef16-0f9b-a095-dc5e66631653%22%2C%22c%22%3A1632334597515%2C%22l%22%3A1632334597515%7D; ajs_anonymous_id=%22310b6ef9-6dca-4bbd-a01f-d821fd351777%22; ajs_user_id=%221544530874%22; _pin_unauth=dWlkPU9HUTNaV0U0TTJZdE1tSXpaUzAwTUdJNUxUZzJaR1l0TWpJeFpUWmtNV1EwTXpGag; _ga=GA1.2.779314607.1632335873; _pin_unauth=dWlkPU9HUTNaV0U0TTJZdE1tSXpaUzAwTUdJNUxUZzJaR1l0TWpJeFpUWmtNV1EwTXpGag; __stripe_mid=3558998b-c888-4c38-99d5-c81e594858989b890e; _clck=y84cf8|1|ev3|0; ab.storage.sessionId.4d3ca359-c724-43de-9b97-cb9f1fee4769=%7B%22g%22%3A%2271ba89b7-7565-7342-8d3c-2713637a6f36%22%2C%22e%22%3A1632759293356%2C%22c%22%3A1632757493357%2C%22l%22%3A1632757493357%7D; amplitude_id_95d3abbefaf19863dc230d5449736018helium10.com=eyJkZXZpY2VJZCI6IjRjMDU0YTAzLWU5NzctNGUyMC1hNjA1LWY1NWVlZmI5MThjMVIiLCJ1c2VySWQiOiIxNTQ0NTMwODc0Iiwib3B0T3V0IjpmYWxzZSwic2Vzc2lvbklkIjoxNjMyNzU3NDk0MzIxLCJsYXN0RXZlbnRUaW1lIjoxNjMyNzU3NDk0MzI4LCJldmVudElkIjo1LCJpZGVudGlmeUlkIjoxMCwic2VxdWVuY2VOdW1iZXIiOjE1fQ==; intercom-session-yzizpoku=aWhQZk5oN243cGx5emE4ejJFS2ZlT2tnWHptdmltSXRBcHQ0VDNxM2lERkZ6cDhsaVIwaWpGSzdMZThkL3psMC0tdGFjZzBwbU81KzFYY1V0MFVaRDlQZz09--b50a42df6570e0c435136b7d541a7e54779dbaa8; _uetvid=1f88ed201bd111ec8403fd5c76943bfc; AWSALB=yvn3bZCVZ573HljOaX9RwwaSKyOt6wGM9WDhSsFNFe3mUekzGQZbwnaeLgtmrWknwkT2p46CqVFoYgGj+r1bdWMKcecl9sYcXul9xG6ZJ8j92dXli/h0/rIqeZ2d; AWSALBCORS=yvn3bZCVZ573HljOaX9RwwaSKyOt6wGM9WDhSsFNFe3mUekzGQZbwnaeLgtmrWknwkT2p46CqVFoYgGj+r1bdWMKcecl9sYcXul9xG6ZJ8j92dXli/h0/rIqeZ2d; sid=tkc11r6685h2oq0l49rn9qvipr; _identity=73000b319de17b36e29829e49e08b824290f94193dece61dbe479cebc1908c5ea%3A2%3A%7Bi%3A0%3Bs%3A9%3A%22_identity%22%3Bi%3A1%3Bs%3A111%3A%22%5B1544530874%2C%22JzVGDOKncfwXvslplFGth3yHKuCSKwwA%22%2C2592000%2C%22nifOJCMOjaPY_nWRH2dXFjYjhtpWCQuw%22%2Cnull%2C%22208.91.53.193%22%5D%22%3B%7D")
        .send()
        .await?;

    // Run a check to make sure we're logged in, stores a cookie I think
    let login_checked = client
        .get("https://members.helium10.com/extension/check-login?accountId=1544530874&appId=njmehopjdpcckochcggncklnlmikcbnb")
        .header(COOKIE, "signUpRequirementType=fe23686b4977cf81a53e69d67e7de24a9c6090131d666abb6c1babecf46de261a%3A2%3A%7Bi%3A0%3Bs%3A21%3A%22signUpRequirementType%22%3Bi%3A1%3Bs%3A20%3A%22phone_input_required%22%3B%7D; _gcl_au=1.1.2052847598.1632334553; _ga=GA1.3.1184175819.1632334553; current-marketplace=5a7deede86e62213f2adae2c21dde37c8919c5317898e27c859a9080214c380fa%3A2%3A%7Bi%3A0%3Bs%3A19%3A%22current-marketplace%22%3Bi%3A1%3Bs%3A13%3A%22ATVPDKIKX0DER%22%3B%7D; ab.storage.userId.4d3ca359-c724-43de-9b97-cb9f1fee4769=%7B%22g%22%3A%221544530874%22%2C%22c%22%3A1632334597512%2C%22l%22%3A1632334597512%7D; ab.storage.deviceId.4d3ca359-c724-43de-9b97-cb9f1fee4769=%7B%22g%22%3A%2260b39bb7-ef16-0f9b-a095-dc5e66631653%22%2C%22c%22%3A1632334597515%2C%22l%22%3A1632334597515%7D; ajs_anonymous_id=%22310b6ef9-6dca-4bbd-a01f-d821fd351777%22; ajs_user_id=%221544530874%22; _pin_unauth=dWlkPU9HUTNaV0U0TTJZdE1tSXpaUzAwTUdJNUxUZzJaR1l0TWpJeFpUWmtNV1EwTXpGag; _ga=GA1.2.779314607.1632335873; _pin_unauth=dWlkPU9HUTNaV0U0TTJZdE1tSXpaUzAwTUdJNUxUZzJaR1l0TWpJeFpUWmtNV1EwTXpGag; __stripe_mid=3558998b-c888-4c38-99d5-c81e594858989b890e; _clck=y84cf8|1|ev3|0; ab.storage.sessionId.4d3ca359-c724-43de-9b97-cb9f1fee4769=%7B%22g%22%3A%2271ba89b7-7565-7342-8d3c-2713637a6f36%22%2C%22e%22%3A1632759293356%2C%22c%22%3A1632757493357%2C%22l%22%3A1632757493357%7D; amplitude_id_95d3abbefaf19863dc230d5449736018helium10.com=eyJkZXZpY2VJZCI6IjRjMDU0YTAzLWU5NzctNGUyMC1hNjA1LWY1NWVlZmI5MThjMVIiLCJ1c2VySWQiOiIxNTQ0NTMwODc0Iiwib3B0T3V0IjpmYWxzZSwic2Vzc2lvbklkIjoxNjMyNzU3NDk0MzIxLCJsYXN0RXZlbnRUaW1lIjoxNjMyNzU3NDk0MzI4LCJldmVudElkIjo1LCJpZGVudGlmeUlkIjoxMCwic2VxdWVuY2VOdW1iZXIiOjE1fQ==; intercom-session-yzizpoku=aWhQZk5oN243cGx5emE4ejJFS2ZlT2tnWHptdmltSXRBcHQ0VDNxM2lERkZ6cDhsaVIwaWpGSzdMZThkL3psMC0tdGFjZzBwbU81KzFYY1V0MFVaRDlQZz09--b50a42df6570e0c435136b7d541a7e54779dbaa8; _uetvid=1f88ed201bd111ec8403fd5c76943bfc; AWSALB=yvn3bZCVZ573HljOaX9RwwaSKyOt6wGM9WDhSsFNFe3mUekzGQZbwnaeLgtmrWknwkT2p46CqVFoYgGj+r1bdWMKcecl9sYcXul9xG6ZJ8j92dXli/h0/rIqeZ2d; AWSALBCORS=yvn3bZCVZ573HljOaX9RwwaSKyOt6wGM9WDhSsFNFe3mUekzGQZbwnaeLgtmrWknwkT2p46CqVFoYgGj+r1bdWMKcecl9sYcXul9xG6ZJ8j92dXli/h0/rIqeZ2d; sid=tkc11r6685h2oq0l49rn9qvipr; _identity=73000b319de17b36e29829e49e08b824290f94193dece61dbe479cebc1908c5ea%3A2%3A%7Bi%3A0%3Bs%3A9%3A%22_identity%22%3Bi%3A1%3Bs%3A111%3A%22%5B1544530874%2C%22JzVGDOKncfwXvslplFGth3yHKuCSKwwA%22%2C2592000%2C%22nifOJCMOjaPY_nWRH2dXFjYjhtpWCQuw%22%2Cnull%2C%22208.91.53.193%22%5D%22%3B%7D")
        .send()
        .await?;

    // Declare that we're going to make a series of requests to a server
    // Sets a cookie that will be used by the server to track the number of requests
    // Returns a hash that we'll use for future product requests.
    let invocation = client
        .post("https://members.helium10.com/black-box/invocations?accountId=1544530874")
        .form(&[
            (("appId"), ("njmehopjdpcckochcggncklnlmikcbnb")),
            (("platform"), ("amazon")),
        ])
        .header("credentials", "include")
        .header(COOKIE, "signUpRequirementType=fe23686b4977cf81a53e69d67e7de24a9c6090131d666abb6c1babecf46de261a%3A2%3A%7Bi%3A0%3Bs%3A21%3A%22signUpRequirementType%22%3Bi%3A1%3Bs%3A20%3A%22phone_input_required%22%3B%7D; _gcl_au=1.1.2052847598.1632334553; _ga=GA1.3.1184175819.1632334553; current-marketplace=5a7deede86e62213f2adae2c21dde37c8919c5317898e27c859a9080214c380fa%3A2%3A%7Bi%3A0%3Bs%3A19%3A%22current-marketplace%22%3Bi%3A1%3Bs%3A13%3A%22ATVPDKIKX0DER%22%3B%7D; ab.storage.userId.4d3ca359-c724-43de-9b97-cb9f1fee4769=%7B%22g%22%3A%221544530874%22%2C%22c%22%3A1632334597512%2C%22l%22%3A1632334597512%7D; ab.storage.deviceId.4d3ca359-c724-43de-9b97-cb9f1fee4769=%7B%22g%22%3A%2260b39bb7-ef16-0f9b-a095-dc5e66631653%22%2C%22c%22%3A1632334597515%2C%22l%22%3A1632334597515%7D; ajs_anonymous_id=%22310b6ef9-6dca-4bbd-a01f-d821fd351777%22; ajs_user_id=%221544530874%22; _pin_unauth=dWlkPU9HUTNaV0U0TTJZdE1tSXpaUzAwTUdJNUxUZzJaR1l0TWpJeFpUWmtNV1EwTXpGag; _ga=GA1.2.779314607.1632335873; _pin_unauth=dWlkPU9HUTNaV0U0TTJZdE1tSXpaUzAwTUdJNUxUZzJaR1l0TWpJeFpUWmtNV1EwTXpGag; __stripe_mid=3558998b-c888-4c38-99d5-c81e594858989b890e; _clck=y84cf8|1|ev3|0; ab.storage.sessionId.4d3ca359-c724-43de-9b97-cb9f1fee4769=%7B%22g%22%3A%2271ba89b7-7565-7342-8d3c-2713637a6f36%22%2C%22e%22%3A1632759293356%2C%22c%22%3A1632757493357%2C%22l%22%3A1632757493357%7D; amplitude_id_95d3abbefaf19863dc230d5449736018helium10.com=eyJkZXZpY2VJZCI6IjRjMDU0YTAzLWU5NzctNGUyMC1hNjA1LWY1NWVlZmI5MThjMVIiLCJ1c2VySWQiOiIxNTQ0NTMwODc0Iiwib3B0T3V0IjpmYWxzZSwic2Vzc2lvbklkIjoxNjMyNzU3NDk0MzIxLCJsYXN0RXZlbnRUaW1lIjoxNjMyNzU3NDk0MzI4LCJldmVudElkIjo1LCJpZGVudGlmeUlkIjoxMCwic2VxdWVuY2VOdW1iZXIiOjE1fQ==; intercom-session-yzizpoku=aWhQZk5oN243cGx5emE4ejJFS2ZlT2tnWHptdmltSXRBcHQ0VDNxM2lERkZ6cDhsaVIwaWpGSzdMZThkL3psMC0tdGFjZzBwbU81KzFYY1V0MFVaRDlQZz09--b50a42df6570e0c435136b7d541a7e54779dbaa8; _uetvid=1f88ed201bd111ec8403fd5c76943bfc; AWSALB=yvn3bZCVZ573HljOaX9RwwaSKyOt6wGM9WDhSsFNFe3mUekzGQZbwnaeLgtmrWknwkT2p46CqVFoYgGj+r1bdWMKcecl9sYcXul9xG6ZJ8j92dXli/h0/rIqeZ2d; AWSALBCORS=yvn3bZCVZ573HljOaX9RwwaSKyOt6wGM9WDhSsFNFe3mUekzGQZbwnaeLgtmrWknwkT2p46CqVFoYgGj+r1bdWMKcecl9sYcXul9xG6ZJ8j92dXli/h0/rIqeZ2d; sid=tkc11r6685h2oq0l49rn9qvipr; _identity=73000b319de17b36e29829e49e08b824290f94193dece61dbe479cebc1908c5ea%3A2%3A%7Bi%3A0%3Bs%3A9%3A%22_identity%22%3Bi%3A1%3Bs%3A111%3A%22%5B1544530874%2C%22JzVGDOKncfwXvslplFGth3yHKuCSKwwA%22%2C2592000%2C%22nifOJCMOjaPY_nWRH2dXFjYjhtpWCQuw%22%2Cnull%2C%22208.91.53.193%22%5D%22%3B%7D")
        .send()
        .await?
        .json::<InvocationResponse>().await?;

    // "fitness rack"
    let ids = vec![
        "B081S9FC5X",
        "B08L8B9JND",
        "B08B8Z6BZK",
        "B081S9FC5X",
        "B08B8Z6BZK",
        "B07XM8P57J",
        "B01N4I8FOY",
        "B01N4I8FOY",
        "B071HT1DH1",
        "B084P72GYX",
        "B08B8Z6BZK",
        "B08L8B9JND",
        "B08LFZGBQ4",
        "B01N4I8FOY",
        "B096X17CX6",
        "B000ASG0VU",
        "B08B8Z6BZK",
        "B07GNQG4C9",
        "B002EJC990",
        "B081S9FC5X",
        "B07CX5KFY9",
        "B07X497Y2L",
        "B08LBQHRRT",
        "B00I04Z52G",
        "B087Q1S9CR",
        "B01NBN4FAK",
        "B01N4I8FOY",
        "B018XDH17K",
        "B079ZJVMMG",
        "B088TYT22H",
        "B01HFL9MUK",
        "B081GS9HG7",
        "B08L8B9JND",
        "B09H6VQZKH",
        "B07F7LVRS4",
        "B07QMP9DYR",
        "B01IRLMUOM",
        "B08287XV7L",
        "B07CHR5QDQ",
        "B08QCY7QNN",
        "B086H6Q399",
        "B07FBDVJN5",
        "B08D327LSN",
        "B07B4Y3WH4",
        "B07B4YXPB4",
        "B07B4YZ9PC",
        "B098TB1NPR",
        "B08NFX16KC",
        "B08NYFX3C7",
        "B07D1JPXJH",
        "B08QBYD9W8",
        "B01N7Z2GGN",
        "B071YP4VPN",
        "B07B6FBYZS",
        "B07MHRT6R5",
        "B07KQZW4JC",
        "B07PVDCMJZ",
        "B09245VLVV",
        "B011NP77PG",
        "B0799RDNRZ",
        "B0932Y9RC1",
        "B07B4DZY6T",
        "B081GS9HG7",
        "B07NZ28QHB",
        "B07SKLTHBB",
        "B08L8B9JND",
        "B07B4ZJCZB",
        "B01IUDX8RU",
        "B00STOHPMU",
        "B08P2WFNCQ",
        "B073XB7H39",
        "B01MZBELJ7",
        "B08DV19XKR",
        "B094R5168C",
        "B09DXTML1F",
        "B08QCY7QNN",
        "B07SM8VJ6P",
    ];

    let out = serde_json::to_string(&ids).unwrap();

    // Query the data of a single product
    // todo: see if we can query multiple products with one call
    let product = client
        .post("https://members.helium10.com/black-box/sales?accountId=1544530874")
        .header(COOKIE, "signUpRequirementType=fe23686b4977cf81a53e69d67e7de24a9c6090131d666abb6c1babecf46de261a%3A2%3A%7Bi%3A0%3Bs%3A21%3A%22signUpRequirementType%22%3Bi%3A1%3Bs%3A20%3A%22phone_input_required%22%3B%7D; _gcl_au=1.1.2052847598.1632334553; _ga=GA1.3.1184175819.1632334553; current-marketplace=5a7deede86e62213f2adae2c21dde37c8919c5317898e27c859a9080214c380fa%3A2%3A%7Bi%3A0%3Bs%3A19%3A%22current-marketplace%22%3Bi%3A1%3Bs%3A13%3A%22ATVPDKIKX0DER%22%3B%7D; ab.storage.userId.4d3ca359-c724-43de-9b97-cb9f1fee4769=%7B%22g%22%3A%221544530874%22%2C%22c%22%3A1632334597512%2C%22l%22%3A1632334597512%7D; ab.storage.deviceId.4d3ca359-c724-43de-9b97-cb9f1fee4769=%7B%22g%22%3A%2260b39bb7-ef16-0f9b-a095-dc5e66631653%22%2C%22c%22%3A1632334597515%2C%22l%22%3A1632334597515%7D; ajs_anonymous_id=%22310b6ef9-6dca-4bbd-a01f-d821fd351777%22; ajs_user_id=%221544530874%22; _pin_unauth=dWlkPU9HUTNaV0U0TTJZdE1tSXpaUzAwTUdJNUxUZzJaR1l0TWpJeFpUWmtNV1EwTXpGag; _ga=GA1.2.779314607.1632335873; _pin_unauth=dWlkPU9HUTNaV0U0TTJZdE1tSXpaUzAwTUdJNUxUZzJaR1l0TWpJeFpUWmtNV1EwTXpGag; __stripe_mid=3558998b-c888-4c38-99d5-c81e594858989b890e; _clck=y84cf8|1|ev3|0; ab.storage.sessionId.4d3ca359-c724-43de-9b97-cb9f1fee4769=%7B%22g%22%3A%2271ba89b7-7565-7342-8d3c-2713637a6f36%22%2C%22e%22%3A1632759293356%2C%22c%22%3A1632757493357%2C%22l%22%3A1632757493357%7D; amplitude_id_95d3abbefaf19863dc230d5449736018helium10.com=eyJkZXZpY2VJZCI6IjRjMDU0YTAzLWU5NzctNGUyMC1hNjA1LWY1NWVlZmI5MThjMVIiLCJ1c2VySWQiOiIxNTQ0NTMwODc0Iiwib3B0T3V0IjpmYWxzZSwic2Vzc2lvbklkIjoxNjMyNzU3NDk0MzIxLCJsYXN0RXZlbnRUaW1lIjoxNjMyNzU3NDk0MzI4LCJldmVudElkIjo1LCJpZGVudGlmeUlkIjoxMCwic2VxdWVuY2VOdW1iZXIiOjE1fQ==; intercom-session-yzizpoku=aWhQZk5oN243cGx5emE4ejJFS2ZlT2tnWHptdmltSXRBcHQ0VDNxM2lERkZ6cDhsaVIwaWpGSzdMZThkL3psMC0tdGFjZzBwbU81KzFYY1V0MFVaRDlQZz09--b50a42df6570e0c435136b7d541a7e54779dbaa8; _uetvid=1f88ed201bd111ec8403fd5c76943bfc; AWSALB=yvn3bZCVZ573HljOaX9RwwaSKyOt6wGM9WDhSsFNFe3mUekzGQZbwnaeLgtmrWknwkT2p46CqVFoYgGj+r1bdWMKcecl9sYcXul9xG6ZJ8j92dXli/h0/rIqeZ2d; AWSALBCORS=yvn3bZCVZ573HljOaX9RwwaSKyOt6wGM9WDhSsFNFe3mUekzGQZbwnaeLgtmrWknwkT2p46CqVFoYgGj+r1bdWMKcecl9sYcXul9xG6ZJ8j92dXli/h0/rIqeZ2d; sid=tkc11r6685h2oq0l49rn9qvipr; _identity=73000b319de17b36e29829e49e08b824290f94193dece61dbe479cebc1908c5ea%3A2%3A%7Bi%3A0%3Bs%3A9%3A%22_identity%22%3Bi%3A1%3Bs%3A111%3A%22%5B1544530874%2C%22JzVGDOKncfwXvslplFGth3yHKuCSKwwA%22%2C2592000%2C%22nifOJCMOjaPY_nWRH2dXFjYjhtpWCQuw%22%2Cnull%2C%22208.91.53.193%22%5D%22%3B%7D")
        .form(&[
            (("asins"), (out.as_str())),
            (("marketplace"), ("ATVPDKIKX0DER")),
            (("hash"), (&invocation.hash)),
            (("platform"), ("amazon")),
        ])
        .send()
        .await?;

    // let product = client
    //     .post("https://members.helium10.com/black-box/sales?accountId=1544530874")
    //     .header(COOKIE, "signUpRequirementType=fe23686b4977cf81a53e69d67e7de24a9c6090131d666abb6c1babecf46de261a%3A2%3A%7Bi%3A0%3Bs%3A21%3A%22signUpRequirementType%22%3Bi%3A1%3Bs%3A20%3A%22phone_input_required%22%3B%7D; _gcl_au=1.1.2052847598.1632334553; _ga=GA1.3.1184175819.1632334553; current-marketplace=5a7deede86e62213f2adae2c21dde37c8919c5317898e27c859a9080214c380fa%3A2%3A%7Bi%3A0%3Bs%3A19%3A%22current-marketplace%22%3Bi%3A1%3Bs%3A13%3A%22ATVPDKIKX0DER%22%3B%7D; ab.storage.userId.4d3ca359-c724-43de-9b97-cb9f1fee4769=%7B%22g%22%3A%221544530874%22%2C%22c%22%3A1632334597512%2C%22l%22%3A1632334597512%7D; ab.storage.deviceId.4d3ca359-c724-43de-9b97-cb9f1fee4769=%7B%22g%22%3A%2260b39bb7-ef16-0f9b-a095-dc5e66631653%22%2C%22c%22%3A1632334597515%2C%22l%22%3A1632334597515%7D; ajs_anonymous_id=%22310b6ef9-6dca-4bbd-a01f-d821fd351777%22; ajs_user_id=%221544530874%22; _pin_unauth=dWlkPU9HUTNaV0U0TTJZdE1tSXpaUzAwTUdJNUxUZzJaR1l0TWpJeFpUWmtNV1EwTXpGag; _ga=GA1.2.779314607.1632335873; _pin_unauth=dWlkPU9HUTNaV0U0TTJZdE1tSXpaUzAwTUdJNUxUZzJaR1l0TWpJeFpUWmtNV1EwTXpGag; __stripe_mid=3558998b-c888-4c38-99d5-c81e594858989b890e; _clck=y84cf8|1|ev3|0; ab.storage.sessionId.4d3ca359-c724-43de-9b97-cb9f1fee4769=%7B%22g%22%3A%2271ba89b7-7565-7342-8d3c-2713637a6f36%22%2C%22e%22%3A1632759293356%2C%22c%22%3A1632757493357%2C%22l%22%3A1632757493357%7D; amplitude_id_95d3abbefaf19863dc230d5449736018helium10.com=eyJkZXZpY2VJZCI6IjRjMDU0YTAzLWU5NzctNGUyMC1hNjA1LWY1NWVlZmI5MThjMVIiLCJ1c2VySWQiOiIxNTQ0NTMwODc0Iiwib3B0T3V0IjpmYWxzZSwic2Vzc2lvbklkIjoxNjMyNzU3NDk0MzIxLCJsYXN0RXZlbnRUaW1lIjoxNjMyNzU3NDk0MzI4LCJldmVudElkIjo1LCJpZGVudGlmeUlkIjoxMCwic2VxdWVuY2VOdW1iZXIiOjE1fQ==; intercom-session-yzizpoku=aWhQZk5oN243cGx5emE4ejJFS2ZlT2tnWHptdmltSXRBcHQ0VDNxM2lERkZ6cDhsaVIwaWpGSzdMZThkL3psMC0tdGFjZzBwbU81KzFYY1V0MFVaRDlQZz09--b50a42df6570e0c435136b7d541a7e54779dbaa8; _uetvid=1f88ed201bd111ec8403fd5c76943bfc; AWSALB=yvn3bZCVZ573HljOaX9RwwaSKyOt6wGM9WDhSsFNFe3mUekzGQZbwnaeLgtmrWknwkT2p46CqVFoYgGj+r1bdWMKcecl9sYcXul9xG6ZJ8j92dXli/h0/rIqeZ2d; AWSALBCORS=yvn3bZCVZ573HljOaX9RwwaSKyOt6wGM9WDhSsFNFe3mUekzGQZbwnaeLgtmrWknwkT2p46CqVFoYgGj+r1bdWMKcecl9sYcXul9xG6ZJ8j92dXli/h0/rIqeZ2d; sid=tkc11r6685h2oq0l49rn9qvipr; _identity=73000b319de17b36e29829e49e08b824290f94193dece61dbe479cebc1908c5ea%3A2%3A%7Bi%3A0%3Bs%3A9%3A%22_identity%22%3Bi%3A1%3Bs%3A111%3A%22%5B1544530874%2C%22JzVGDOKncfwXvslplFGth3yHKuCSKwwA%22%2C2592000%2C%22nifOJCMOjaPY_nWRH2dXFjYjhtpWCQuw%22%2Cnull%2C%22208.91.53.193%22%5D%22%3B%7D")
    //     .form(&[
    //         (("asins"), ("[\"B08NK5FBWV\"]")),
    //         (("marketplace"), ("ATVPDKIKX0DER")),
    //         (("hash"), (&invocation.hash)),
    //         (("platform"), ("amazon")),
    //     ])
    //     .send()
    //     .await?;
    dbg!(&product);
    // dbg!();
    let contents = product.text().await?;

    dbg!(&contents);
    std::fs::write("./data/amazon_response.json", contents).unwrap();

    Ok(())
}
