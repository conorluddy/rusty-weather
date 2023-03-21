use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct WeatherData {
    pub geometry: Geometry,
    pub properties: Properties,
    pub r#type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Geometry {
    pub coordinates: [f64; 3],
    pub r#type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Properties {
    pub meta: Meta,
    pub timeseries: Vec<Timeseries>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
    pub units: Units,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Units {
    pub air_pressure_at_sea_level: Option<String>,
    pub air_temperature: Option<String>,
    pub air_temperature_max: Option<String>,
    pub air_temperature_min: Option<String>,
    pub cloud_area_fraction: Option<String>,
    pub cloud_area_fraction_high: Option<String>,
    pub cloud_area_fraction_low: Option<String>,
    pub cloud_area_fraction_medium: Option<String>,
    pub dew_point_temperature: Option<String>,
    pub fog_area_fraction: Option<String>,
    pub precipitation_amount: Option<String>,
    pub precipitation_amount_max: Option<String>,
    pub precipitation_amount_min: Option<String>,
    pub probability_of_precipitation: Option<String>,
    pub probability_of_thunder: Option<String>,
    pub relative_humidity: Option<String>,
    pub ultraviolet_index_clear_sky_max: Option<String>,
    pub wind_from_direction: Option<String>,
    pub wind_speed: Option<String>,
    pub wind_speed_of_gust: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Timeseries {
    pub data: Data,
    pub time: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub instant: Instant,
    pub next_1_hours: Option<Next1Hours>,
    pub next_6_hours: Option<Next6Hours>,
    pub next_12_hours: Option<Next12Hours>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Instant {
    pub details: InstantDetails,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InstantDetails {
    pub air_pressure_at_sea_level: Option<f64>,
    pub air_temperature: Option<f64>,
    pub cloud_area_fraction: Option<f64>,
    pub cloud_area_fraction_high: Option<f64>,
    pub cloud_area_fraction_low: Option<f64>,
    pub cloud_area_fraction_medium: Option<f64>,
    pub dew_point_temperature: Option<f64>,
    pub fog_area_fraction: Option<f64>,
    pub relative_humidity: Option<f64>,
    pub wind_from_direction: Option<f64>,
    pub wind_speed: Option<f64>,
    pub wind_speed_of_gust: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Next12Hours {
    pub summary: Summary,
    pub details: Option<Details>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Next1Hours {
    pub summary: Summary,
    pub details: Option<Details>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Next6Hours {
    pub summary: Summary,
    pub details: Option<Details>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Summary {
    pub symbol_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Details {
    pub air_temperature_max: Option<f64>,
    pub air_temperature_min: Option<f64>,
    pub precipitation_amount: Option<f64>,
    pub precipitation_amount_max: Option<f64>,
    pub precipitation_amount_min: Option<f64>,
    pub probability_of_precipitation: Option<f64>,
    pub probability_of_thunder: Option<f64>,
    pub ultraviolet_index_clear_sky_max: Option<f64>,
}
