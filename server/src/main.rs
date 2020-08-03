// mod ffxiv_data;

use std::io::Cursor;

use futures::future::BoxFuture;
use rocket::{
    fairing::AdHoc,
    get,
    http::Header,
    launch,
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

fn get_allowed_origin(source_origin: Option<&str>) -> &str {
    const ALLOWD_ORIGINS: [&str; 2] = ["https://ffxiv-dev.dlunch.net", "http://localhost:8080"];

    if let Some(origin) = source_origin {
        if ALLOWD_ORIGINS.iter().any(|&x| x == origin) {
            &origin
        } else {
            "https://ffxiv.dlunch.net"
        }
    } else {
        "https://ffxiv.dlunch.net"
    }
}

fn insert_headers<'a>(response: &'a mut Response, allowed_origin: &'a str) {
    response.set_header(Header::new("Access-Control-Allow-Origin", allowed_origin.to_owned()));
    response.set_header(Header::new("Access-Control-Allow-Methods", "GET"));
    response.set_header(Header::new("Access-Control-Allow-Headers", "Content-Type"));
    response.set_header(Header::new("Vary", "Origin, Accept-Encoding"));
}

fn attach_cors<'a, 'r, 's>(req: &'a Request<'r>, mut res: &'a mut Response<'s>) -> BoxFuture<'a, ()> {
    Box::pin(async move {
        let source_origin = req.headers().get_one("Origin");
        let allowed_origin = get_allowed_origin(source_origin);

        insert_headers(&mut res, allowed_origin);
    })
}

#[launch]
fn rocket() -> rocket::Rocket {
    pretty_env_logger::formatted_timed_builder().filter_level(log::LevelFilter::Debug).init();

    rocket::ignite()
        .attach(AdHoc::on_response("CORS", attach_cors))
        .mount("/", routes![probe])
}
