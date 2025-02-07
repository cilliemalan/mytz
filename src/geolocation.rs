use crate::tz::lookup_tz_posix_string;

use lru::LruCache;
use once_cell::sync::Lazy;
use std::num::NonZeroUsize;
use std::sync::Mutex;

static GEO_CACHE: Lazy<Mutex<LruCache<String, String>>> =
    Lazy::new(|| Mutex::new(LruCache::new(NonZeroUsize::new(100 * 1024).unwrap())));

#[derive(Serialize, Deserialize)]
pub struct TimezoneResult {
    geo: Geo,
    timezone: String,
    timezone_offset: i64,
    timezone_offset_with_dst: i64,
    date: String,
    date_time: String,
    date_time_txt: String,
    date_time_wti: String,
    date_time_ymd: String,
    date_time_unix: f64,
    time_24: String,
    time_12: String,
    week: i64,
    month: i64,
    year: i64,
    year_abbr: String,
    is_dst: bool,
    dst_savings: i64,
    dst_exists: bool,
    dst_start: String,
    dst_end: String,
}

#[derive(Serialize, Deserialize)]
pub struct Geo {
    continent_code: String,
    continent_name: String,
    country_code2: String,
    country_code3: String,
    country_name: String,
    country_name_official: String,
    is_eu: bool,
    state_prov: String,
    state_code: String,
    district: String,
    city: String,
    zipcode: String,
    latitude: String,
    longitude: String,
}

static API_URL: &str = "https://api.ipgeolocation.io/timezone";

pub async fn geolocate(api_key: String, ip_address: String) -> reqwest::Result<String> {
    let cached = GEO_CACHE
        .lock()
        .ok()
        .map(|mut cache| {
            cache.get(ip_address.as_str()).map(|s| s.clone())
        })
        .flatten();

    if let Some(cached_value) = cached {
        return Ok(cached_value);
    }
    
    let client = reqwest::Client::new();
    let ip_addressu8 = urlencoding::encode(&ip_address);
    let request_url = format!("{}?apiKey={}&ip={}", API_URL, api_key, ip_addressu8);

    let response = client
        .get(&request_url)
        .send()
        .await?
        .json::<TimezoneResult>()
        .await?;

    GEO_CACHE.lock()
        .map(|mut cache| {
            cache.put(ip_address,response.timezone.clone());
        }).ok();

    Ok(response.timezone)
}

pub async fn geolocate_timezone(api_key: String, ip_address: String) -> Option<String> {
    geolocate(api_key, ip_address).await.ok()
}

pub async fn geolocate_tz(api_key: String, ip_address: String) -> Option<&'static str> {
    geolocate(api_key, ip_address)
        .await
        .ok()
        .map(|r| lookup_tz_posix_string(r.as_str()))
        .flatten()
}
