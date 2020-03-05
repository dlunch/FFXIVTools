use std::convert::Infallible;
use std::io;

use dotenv::dotenv;
use warp::Filter;

async fn get_exl() -> Result<impl warp::Reply, Infallible> {
    Ok("")
}

#[tokio::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let exl = warp::get().and(warp::path!("parsed" / "exl")).and_then(get_exl);
    let routes = exl;

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;

    Ok(())
}
