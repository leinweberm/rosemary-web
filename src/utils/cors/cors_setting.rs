pub fn settings() -> warp::cors::Builder {
    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "POST", "PATCH", "PUT", "DELETE"])
				.allow_credentials(true);
				// .allow_headers("*");
        // .allow_headers(vec![
        //     "Content-Type",
        //     "content-type",
        //     "Content-Length",
        //     "content-length",
        //     "Authorization",
        //     "authorization",
        //     "User-Agent",
        //     "Referer",
        //     "Origin",
        //     "Sec-Fetch-Mode",
        //     "Access-Control-Request-Method",
        //     "Access-Control-Request-Headers",
        //     "Access-Control-Allow-Origin",
        // ]);

    cors
}