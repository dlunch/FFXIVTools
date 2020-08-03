// mod ffxiv_data;

use std::io::Cursor;

use rocket::{
    get, launch,
    request::{FromRequest, Outcome},
    routes, Request, Response,
};

struct CloudFlareHeader {
    dc: Option<String>,
    ip_country: Option<String>,
}

#[rocket::async_trait]
impl<'a, 'r> FromRequest<'a, 'r> for CloudFlareHeader {
    type Error = ();

    async fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let dc = request.headers().get_one("CF_RAY").map(|x| x.split('-').nth(1).unwrap().to_owned());
        let ip_country = request.headers().get_one("CF_IPCOUNTRY").map(|x| x.to_owned());

        Outcome::Success(Self { dc, ip_country })
    }
}

#[get("/probe")]
fn probe<'r>(header: CloudFlareHeader) -> Response<'r> {
    let enable_cf = if let (Some(dc), Some(ip_country)) = (header.dc, header.ip_country) {
        !(ip_country == "KR" && dc != "ICN")
    } else {
        true
    };

    let response = if enable_cf {
        "https://ffxiv-data.dlunch.net"
    } else {
        "https://ffxiv-data3.dlunch.net"
    };

    Response::build()
        .raw_header("Cache-Control", "max-age=31536000") // TODO
        .sized_body(response.len(), Cursor::new(response))
        .finalize()
}

/*
fn get_allowed_origin(source_origin: Option<&HeaderValue>) -> HeaderValue {
    const ALLOWD_ORIGINS: [&str; 2] = ["https://ffxiv-dev.dlunch.net", "http://localhost:8080"];

    if let Some(origin) = source_origin {
        if ALLOWD_ORIGINS.iter().any(|x| x == origin) {
            origin.to_owned()
        } else {
            HeaderValue::from_static("https://ffxiv.dlunch.net")
        }
    } else {
        HeaderValue::from_static("https://ffxiv.dlunch.net")
    }
}

fn insert_headers(header_map: &mut HeaderMap, allowed_origin: HeaderValue) {
    header_map.insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, allowed_origin);
    header_map.insert(header::ACCESS_CONTROL_ALLOW_METHODS, HeaderValue::from_static("GET"));
    header_map.insert(header::ACCESS_CONTROL_ALLOW_HEADERS, HeaderValue::from_static("Content-Type"));
    header_map.insert(header::VARY, HeaderValue::from_static("Origin, Accept-Encoding"));
}
*/

#[launch]
fn rocket() -> rocket::Rocket {
    pretty_env_logger::formatted_timed_builder().filter_level(log::LevelFilter::Debug).init();

    rocket::ignite().mount("/", routes![probe])
}
