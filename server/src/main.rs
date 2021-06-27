mod ffxiv_data;

use futures::future::BoxFuture;
use rocket::{
    fairing::AdHoc,
    get,
    http::Header,
    launch,
    request::{FromRequest, Outcome},
    routes, Request, Responder, Response,
};

struct CloudFlareHeader {
    dc: Option<String>,
    ip_country: Option<String>,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for CloudFlareHeader {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let dc = request.headers().get_one("CF_RAY").map(|x| x.split('-').nth(1).unwrap().to_owned());
        let ip_country = request.headers().get_one("CF_IPCOUNTRY").map(|x| x.to_owned());

        Outcome::Success(Self { dc, ip_country })
    }
}

#[derive(Responder)]
#[response(content_type = "text")]
struct ProbeResponse {
    body: &'static str,
    header: Header<'static>,
}

#[get("/probe")]
fn probe<'r>(header: CloudFlareHeader) -> ProbeResponse {
    let enable_cf = if let (Some(dc), Some(ip_country)) = (header.dc, header.ip_country) {
        !(ip_country == "KR" && dc != "ICN")
    } else {
        true
    };

    let response = if enable_cf {
        "https://ffxiv-data.dlunch.net"
    } else {
        "https://ffxiv-data-kr.dlunch.net"
    };

    ProbeResponse {
        body: response,
        header: Header::new("Cache-Control", "max-age=31536000"),
    }
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

fn attach_cors<'b, 'r>(req: &'r Request<'_>, mut res: &'b mut Response<'r>) -> BoxFuture<'b, ()> {
    Box::pin(async move {
        let source_origin = req.headers().get_one("Origin");
        let allowed_origin = get_allowed_origin(source_origin);

        insert_headers(&mut res, allowed_origin);
    })
}

#[launch]
fn rocket() -> _ {
    pretty_env_logger::init_timed();

    rocket::build()
        .attach(AdHoc::on_response("CORS", attach_cors))
        .attach(AdHoc::on_ignite("ffxiv_data", ffxiv_data::config))
        .mount("/", routes![probe])
}
