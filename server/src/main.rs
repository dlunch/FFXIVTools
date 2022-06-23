mod ffxiv_data;

use std::{error::Error, net::SocketAddr};

use axum::{
    headers::{self, Header, HeaderName, HeaderValue},
    response::{AppendHeaders, IntoResponse},
    routing::get,
    Router, TypedHeader,
};
use http::{
    header::{self, VARY},
    Method,
};
use tower::ServiceBuilder;
use tower_http::{catch_panic::CatchPanicLayer, cors::CorsLayer, set_header::SetResponseHeaderLayer};

struct CfRay {
    _id: String,
    dc: String,
}

static CF_RAY: HeaderName = HeaderName::from_static("cf-ray");
impl Header for CfRay {
    fn name() -> &'static HeaderName {
        &CF_RAY
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, headers::Error>
    where
        I: Iterator<Item = &'i HeaderValue>,
    {
        let value = values.next().ok_or_else(headers::Error::invalid)?;

        let split = value.to_str().map_err(|_| headers::Error::invalid())?.split('-').collect::<Vec<_>>();
        if split.len() != 2 {
            return Err(headers::Error::invalid());
        }

        Ok(Self {
            _id: split[0].to_string(),
            dc: split[1].to_string(),
        })
    }

    fn encode<E>(&self, _: &mut E)
    where
        E: Extend<HeaderValue>,
    {
        unimplemented!()
    }
}

struct CfIpCountry(String);

static CF_IPCOUNTRY: HeaderName = HeaderName::from_static("cf-ipcountry");
impl Header for CfIpCountry {
    fn name() -> &'static HeaderName {
        &CF_IPCOUNTRY
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, headers::Error>
    where
        I: Iterator<Item = &'i HeaderValue>,
    {
        let value = values
            .next()
            .ok_or_else(headers::Error::invalid)?
            .to_str()
            .map_err(|_| headers::Error::invalid())?;

        Ok(Self(value.to_string()))
    }

    fn encode<E>(&self, _: &mut E)
    where
        E: Extend<HeaderValue>,
    {
        unimplemented!()
    }
}

async fn probe(cf_ray: Option<TypedHeader<CfRay>>, cf_ipcountry: Option<TypedHeader<CfIpCountry>>) -> impl IntoResponse {
    let enable_cf = if let (Some(TypedHeader(cf_ray)), Some(TypedHeader(cf_ipcountry))) = (cf_ray, cf_ipcountry) {
        !(cf_ipcountry.0 == "KR" && cf_ray.dc != "ICN")
    } else {
        true
    };

    let url = if enable_cf {
        "https://ffxiv-data.dlunch.net"
    } else {
        "https://ffxiv-data-kr.dlunch.net"
    };

    (AppendHeaders([(VARY, "CF-ray, CF-IPCountry")]), url)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init_timed();

    let origins = vec![
        "https://ffxiv.dlunch.net".parse()?,
        "https://ffxiv-dev.dlunch.net".parse()?,
        "http://localhost:8080".parse()?,
    ];

    let app = Router::new().route("/probe", get(probe)).merge(ffxiv_data::router()).layer(
        ServiceBuilder::new()
            .layer(
                CorsLayer::new()
                    .allow_origin(origins)
                    .allow_methods(vec![Method::GET])
                    .allow_headers(vec![header::CONTENT_TYPE]),
            )
            .layer(SetResponseHeaderLayer::if_not_present(
                header::CACHE_CONTROL,
                HeaderValue::from_static("public,max-age=31536000"),
            ))
            .layer(CatchPanicLayer::new()),
    );

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    axum::Server::bind(&addr).serve(app.into_make_service()).await?;

    Ok(())
}
