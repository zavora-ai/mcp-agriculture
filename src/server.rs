use rmcp::{handler::server::wrapper::Parameters, tool, tool_router};
use serde_json::json;

use crate::agtech::AgTechClient;
use crate::types::*;

#[derive(Clone)]
pub struct AgricultureServer {
    pub client: AgTechClient,
}

macro_rules! ok_or_err {
    ($e:expr) => {
        match $e {
            Ok(v) => serde_json::to_string_pretty(&v).unwrap_or_default(),
            Err(e) => format!("Error: {e}"),
        }
    };
}

#[tool_router(server_handler)]
impl AgricultureServer {
    // ─── Fields (5) ──────────────────────────────────────────────────────────

    #[tool(description = "List all farm fields/plots")]
    async fn list_fields(&self) -> String {
        ok_or_err!(self.client.get("/fields").await)
    }

    #[tool(description = "Get field details including area, location, soil type, and current crop")]
    async fn get_field(&self, Parameters(input): Parameters<GetFieldInput>) -> String {
        ok_or_err!(self.client.get(&format!("/fields/{}", input.id)).await)
    }

    #[tool(description = "Register a new field/plot")]
    async fn create_field(&self, Parameters(input): Parameters<CreateFieldInput>) -> String {
        let body = json!({"name": input.name, "area_hectares": input.area_hectares, "lat": input.lat, "lon": input.lon, "soil_type": input.soil_type, "elevation_m": input.elevation_m, "irrigation": input.irrigation, "region": input.region});
        ok_or_err!(self.client.post("/fields", &body).await)
    }

    #[tool(description = "Update field details")]
    async fn update_field(&self, Parameters(input): Parameters<UpdateFieldInput>) -> String {
        let body = json!({"name": input.name, "soil_type": input.soil_type, "irrigation": input.irrigation});
        ok_or_err!(self.client.patch(&format!("/fields/{}", input.id), &body).await)
    }

    #[tool(description = "Remove a field from the farm")]
    async fn delete_field(&self, Parameters(input): Parameters<DeleteFieldInput>) -> String {
        ok_or_err!(self.client.delete(&format!("/fields/{}", input.id)).await)
    }

    // ─── Crops (5) ───────────────────────────────────────────────────────────

    #[tool(description = "List crops planted across all fields")]
    async fn list_crops(&self, Parameters(input): Parameters<ListCropsInput>) -> String {
        let mut params = vec![];
        let fid;
        if let Some(ref f) = input.field_id { fid = f.clone(); params.push(("field_id", fid.as_str())); }
        ok_or_err!(self.client.get_query("/crops", &params).await)
    }

    #[tool(description = "Get crop details including growth stage, planting date, and expected harvest")]
    async fn get_crop(&self, Parameters(input): Parameters<GetCropInput>) -> String {
        ok_or_err!(self.client.get(&format!("/crops/{}", input.id)).await)
    }

    #[tool(description = "Record a new crop planting on a field")]
    async fn plant_crop(&self, Parameters(input): Parameters<PlantCropInput>) -> String {
        let body = json!({"field_id": input.field_id, "crop_name": input.crop_name, "variety": input.variety, "expected_harvest": input.expected_harvest});
        ok_or_err!(self.client.post("/crops", &body).await)
    }

    #[tool(description = "Get recommended planting/harvesting calendar for a crop in a region")]
    async fn get_crop_calendar(&self, Parameters(input): Parameters<CropCalendarInput>) -> String {
        let mut params = vec![];
        let c; let r;
        if let Some(ref v) = input.crop { c = v.clone(); params.push(("crop", c.as_str())); }
        if let Some(ref v) = input.region { r = v.clone(); params.push(("region", r.as_str())); }
        ok_or_err!(self.client.get_query("/crops/calendar", &params).await)
    }

    #[tool(description = "Record a harvest with yield data")]
    async fn log_harvest(&self, Parameters(input): Parameters<LogHarvestInput>) -> String {
        let body = json!({"field_id": input.field_id, "crop_id": input.crop_id, "crop_name": input.crop_name, "yield_kg": input.yield_kg, "quality": input.quality});
        ok_or_err!(self.client.post("/harvests", &body).await)
    }

    // ─── Activities (3) ──────────────────────────────────────────────────────

    #[tool(description = "Log a farm activity (irrigation, fertilizer, spraying, weeding)")]
    async fn log_activity(&self, Parameters(input): Parameters<LogActivityInput>) -> String {
        let body = json!({"field_id": input.field_id, "activity_type": input.activity_type, "description": input.description, "cost": input.cost});
        ok_or_err!(self.client.post("/activities", &body).await)
    }

    #[tool(description = "List farm activities with optional field/date filter")]
    async fn list_activities(&self, Parameters(input): Parameters<ListActivitiesInput>) -> String {
        let mut params = vec![];
        let fid;
        if let Some(ref f) = input.field_id { fid = f.clone(); params.push(("field_id", fid.as_str())); }
        ok_or_err!(self.client.get_query("/activities", &params).await)
    }

    #[tool(description = "Get activity summary and costs for a period")]
    async fn get_activity_summary(&self) -> String {
        ok_or_err!(self.client.get("/activities/summary").await)
    }

    // ─── Weather (4) ─────────────────────────────────────────────────────────

    #[tool(description = "Get 7-day weather forecast for a field location")]
    async fn get_forecast(&self, Parameters(input): Parameters<LocationInput>) -> String {
        let mut params = vec![];
        let lat_s; let lon_s; let fid;
        if let Some(v) = input.lat { lat_s = v.to_string(); params.push(("lat", lat_s.as_str())); }
        if let Some(v) = input.lon { lon_s = v.to_string(); params.push(("lon", lon_s.as_str())); }
        if let Some(ref f) = input.field_id { fid = f.clone(); params.push(("field_id", fid.as_str())); }
        ok_or_err!(self.client.get_query("/weather/forecast", &params).await)
    }

    #[tool(description = "Get historical weather data for a date range")]
    async fn get_historical_weather(&self, Parameters(input): Parameters<LocationInput>) -> String {
        let mut params = vec![];
        let lat_s; let lon_s; let fid;
        if let Some(v) = input.lat { lat_s = v.to_string(); params.push(("lat", lat_s.as_str())); }
        if let Some(v) = input.lon { lon_s = v.to_string(); params.push(("lon", lon_s.as_str())); }
        if let Some(ref f) = input.field_id { fid = f.clone(); params.push(("field_id", fid.as_str())); }
        ok_or_err!(self.client.get_query("/weather/historical", &params).await)
    }

    #[tool(description = "Get active weather alerts (frost, drought, heavy rain)")]
    async fn get_weather_alerts(&self) -> String {
        ok_or_err!(self.client.get("/weather/alerts").await)
    }

    #[tool(description = "Calculate growing degree days (GDD) for crop maturity estimation")]
    async fn get_growing_degree_days(&self, Parameters(input): Parameters<GddInput>) -> String {
        let mut params = vec![];
        let c;
        if let Some(ref v) = input.commodity { c = v.clone(); params.push(("commodity", c.as_str())); }
        ok_or_err!(self.client.get_query("/weather/gdd", &params).await)
    }

    // ─── Satellite (4) ───────────────────────────────────────────────────────

    #[tool(description = "Get NDVI (vegetation index) for a field — indicates crop health")]
    async fn get_field_ndvi(&self, Parameters(input): Parameters<LocationInput>) -> String {
        let mut params = vec![];
        let fid;
        if let Some(ref f) = input.field_id { fid = f.clone(); params.push(("field_id", fid.as_str())); }
        ok_or_err!(self.client.get_query("/satellite/ndvi", &params).await)
    }

    #[tool(description = "Get crop health assessment from satellite imagery")]
    async fn get_crop_health(&self, Parameters(input): Parameters<LocationInput>) -> String {
        let mut params = vec![];
        let fid;
        if let Some(ref f) = input.field_id { fid = f.clone(); params.push(("field_id", fid.as_str())); }
        ok_or_err!(self.client.get_query("/satellite/health", &params).await)
    }

    #[tool(description = "Detect field anomalies (water stress, pest damage, nutrient deficiency)")]
    async fn detect_anomalies(&self, Parameters(input): Parameters<LocationInput>) -> String {
        let mut params = vec![];
        let fid;
        if let Some(ref f) = input.field_id { fid = f.clone(); params.push(("field_id", fid.as_str())); }
        ok_or_err!(self.client.get_query("/satellite/anomalies", &params).await)
    }

    #[tool(description = "Get field boundary polygon from satellite")]
    async fn get_field_boundary(&self, Parameters(input): Parameters<LocationInput>) -> String {
        let mut params = vec![];
        let fid;
        if let Some(ref f) = input.field_id { fid = f.clone(); params.push(("field_id", fid.as_str())); }
        ok_or_err!(self.client.get_query("/satellite/boundary", &params).await)
    }

    // ─── Market (5) ──────────────────────────────────────────────────────────

    #[tool(description = "Get current price for a commodity (coffee, tea, maize, wheat)")]
    async fn get_commodity_price(&self, Parameters(input): Parameters<CommodityInput>) -> String {
        let mut params = vec![];
        let c;
        if let Some(ref v) = input.commodity { c = v.clone(); params.push(("commodity", c.as_str())); }
        ok_or_err!(self.client.get_query("/market/price", &params).await)
    }

    #[tool(description = "Get historical price data for a commodity")]
    async fn get_price_history(&self, Parameters(input): Parameters<CommodityInput>) -> String {
        let mut params = vec![];
        let c; let p;
        if let Some(ref v) = input.commodity { c = v.clone(); params.push(("commodity", c.as_str())); }
        if let Some(ref v) = input.period { p = v.clone(); params.push(("period", p.as_str())); }
        ok_or_err!(self.client.get_query("/market/history", &params).await)
    }

    #[tool(description = "Get market trend analysis and price predictions")]
    async fn get_market_trends(&self, Parameters(input): Parameters<CommodityInput>) -> String {
        let mut params = vec![];
        let c;
        if let Some(ref v) = input.commodity { c = v.clone(); params.push(("commodity", c.as_str())); }
        ok_or_err!(self.client.get_query("/market/trends", &params).await)
    }

    #[tool(description = "List available commodities with current prices")]
    async fn list_commodities(&self) -> String {
        ok_or_err!(self.client.get("/market/commodities").await)
    }

    #[tool(description = "Get recommended best time to sell based on price trends and storage costs")]
    async fn get_best_sell_time(&self, Parameters(input): Parameters<CommodityInput>) -> String {
        let mut params = vec![];
        let c;
        if let Some(ref v) = input.commodity { c = v.clone(); params.push(("commodity", c.as_str())); }
        ok_or_err!(self.client.get_query("/market/best-sell-time", &params).await)
    }

    // ─── IoT (4) ─────────────────────────────────────────────────────────────

    #[tool(description = "List IoT sensors deployed on the farm")]
    async fn list_sensors(&self) -> String {
        ok_or_err!(self.client.get("/iot/sensors").await)
    }

    #[tool(description = "Get latest reading from a specific sensor")]
    async fn get_sensor_reading(&self, Parameters(input): Parameters<SensorInput>) -> String {
        ok_or_err!(self.client.get(&format!("/iot/sensors/{}", input.id)).await)
    }

    #[tool(description = "Get soil moisture readings for a field")]
    async fn get_soil_moisture(&self, Parameters(input): Parameters<LocationInput>) -> String {
        let mut params = vec![];
        let fid;
        if let Some(ref f) = input.field_id { fid = f.clone(); params.push(("field_id", fid.as_str())); }
        ok_or_err!(self.client.get_query("/iot/soil-moisture", &params).await)
    }

    #[tool(description = "Get rainfall data from rain gauges")]
    async fn get_rainfall(&self, Parameters(input): Parameters<LocationInput>) -> String {
        let mut params = vec![];
        let fid;
        if let Some(ref f) = input.field_id { fid = f.clone(); params.push(("field_id", fid.as_str())); }
        ok_or_err!(self.client.get_query("/iot/rainfall", &params).await)
    }

    // ─── Yield & Alerts (5) ──────────────────────────────────────────────────

    #[tool(description = "Estimate expected yield based on weather, NDVI, and historical data")]
    async fn estimate_yield(&self, Parameters(input): Parameters<LocationInput>) -> String {
        let mut params = vec![];
        let fid;
        if let Some(ref f) = input.field_id { fid = f.clone(); params.push(("field_id", fid.as_str())); }
        ok_or_err!(self.client.get_query("/yield/estimate", &params).await)
    }

    #[tool(description = "Get historical harvest data for a field")]
    async fn get_harvest_history(&self, Parameters(input): Parameters<LocationInput>) -> String {
        let mut params = vec![];
        let fid;
        if let Some(ref f) = input.field_id { fid = f.clone(); params.push(("field_id", fid.as_str())); }
        ok_or_err!(self.client.get_query("/harvests", &params).await)
    }

    #[tool(description = "Compare current season performance against previous seasons")]
    async fn compare_seasons(&self, Parameters(input): Parameters<LocationInput>) -> String {
        let mut params = vec![];
        let fid;
        if let Some(ref f) = input.field_id { fid = f.clone(); params.push(("field_id", fid.as_str())); }
        ok_or_err!(self.client.get_query("/yield/compare", &params).await)
    }

    #[tool(description = "Get pest and disease alerts for the region")]
    async fn get_pest_alerts(&self) -> String {
        ok_or_err!(self.client.get("/alerts/pests").await)
    }

    #[tool(description = "Get disease risk assessment based on weather and crop conditions")]
    async fn get_disease_risk(&self, Parameters(input): Parameters<LocationInput>) -> String {
        let mut params = vec![];
        let fid;
        if let Some(ref f) = input.field_id { fid = f.clone(); params.push(("field_id", fid.as_str())); }
        ok_or_err!(self.client.get_query("/alerts/disease-risk", &params).await)
    }
}
