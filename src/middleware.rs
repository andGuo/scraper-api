use axum::{
    http::{ Request, HeaderValue, header::CONTENT_TYPE},
    response::{Response, Redirect},
    middleware::Next,
};

// Define a custom middleware that checks the Content-Type header and redirects if it matches.
pub async fn redirect_middleware<B>(
    req: Request<B>,
    next: Next<B>) -> Result<Response, Redirect> {
        let headers = req.headers();
        let default_header = HeaderValue::from_static("application/json");
        let content = headers.get(CONTENT_TYPE).unwrap_or(&default_header).to_str().unwrap_or_default();
    if content.contains("http") {
        let path = req.uri().path();
        let query = req.uri().query().map(|q| format!("?{}", q)).unwrap_or_default();

        // Construct the redirect URL by appending the path and query parameters to the frontend URL.
        let redirect_url = format!(
            "{}{}{}",
            std::env::var("FRONTEND_URL").expect("FRONTEND_URL must be set."),
            path,
            query
        );
        println!("redirecting to {}", redirect_url);
        return Err(Redirect::permanent(&redirect_url))
    }
    Ok(next.run(req).await)
}

