use worker::*;

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let router = Router::new();

    router
        // 1. Flight status (Cached for 60 minutes)
        .get_async("/flights/Number/*path", |req, ctx| async move {
            let path = ctx.param("path").unwrap();
            fetch_and_cache(req, &ctx.env, format!("/flights/number/{}", path), 3600).await
        })
        // 2. Everything else (No cache)
        .get_async("/flights/Airports/*path", |req, ctx| async move {
            let path = ctx.param("path").unwrap();
            fetch_no_cache(req, &ctx.env, format!("/flights/airports/{}", path)).await
        })
        .run(req, env)
        .await
}

/// Forwards query parameters from the incoming request `req` to the upstream
/// `api_path`.  Returns the fully-qualified upstream URL.
fn build_upstream_url(req: &Request, api_path: &str) -> String {
    let base = format!("https://aerodatabox.p.rapidapi.com{}", api_path);

    if let Ok(url) = req.url() {
        let query = url.query();
        match query {
            Some(q) if !q.is_empty() => format!("{}?{}", base, q),
            _ => base,
        }
    } else {
        base
    }
}

async fn fetch_and_cache(req: Request, env: &Env, api_path: String, ttl: u32) -> Result<Response> {
    let cache = Cache::default();
    let url = req.url()?;

    // Use the full original URL (with query params) as the cache key so
    // different param combinations don't share stale entries.
    let cache_key = url.to_string();

    // Check Cache
    if let Some(cached) = cache.get(&cache_key, true).await? {
        return Ok(cached);
    }

    // Miss: Fetch from adb
    let resp = fetch_no_cache(req, env, api_path).await?;

    let new_headers = resp.headers().clone();
    new_headers.set("Cache-Control", &format!("s-maxage={}", ttl))?;

    let mut new_resp = Response::from_body(resp.body().clone())?.with_headers(new_headers);
    cache.put(&cache_key, new_resp.cloned()?).await?;

    Ok(new_resp)
}

async fn fetch_no_cache(req: Request, env: &Env, api_path: String) -> Result<Response> {
    let target_url = build_upstream_url(&req, &api_path);

    let headers = Headers::new();
    headers.set("x-rapidapi-key", &env.var("AERODATABOX_KEY")?.to_string())?;
    headers.set("x-rapidapi-host", "aerodatabox.p.rapidapi.com");

    let mut init = RequestInit::new();
    init.with_method(Method::Get);
    init.with_headers(headers);

    let request = Request::new_with_init(&target_url, &init)?;
    Fetch::Request(request).send().await
}
