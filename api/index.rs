use http::StatusCode;
use rand::{thread_rng, Rng};
use std::error::Error;
use vercel_lambda::{error::VercelError, lambda, IntoResponse, Request, Response};

fn handler(_: Request) -> Result<impl IntoResponse, VercelError> {
    let mut rng = thread_rng();

    let response = Response::builder()
        .status(StatusCode::FOUND)
        .header("Location", format!("/{}.png", rng.gen_range(1..7)))
        // https://vercel.com/docs/concepts/edge-network/caching
        .header("Cache-Control", "no-cache")
        .body("")
        .expect("Internal Server Error");

    Ok(response)
}

fn main() -> Result<(), Box<dyn Error>> {
    Ok(lambda!(handler))
}
