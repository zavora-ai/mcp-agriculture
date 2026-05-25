//! AgTech Example Backend — Farm Management + Intelligence API
//! Provides a complete REST API with seeded Kenyan farm data.
//! Run: cd example-backend && cargo run
//! API: http://localhost:7800/api/v1

use axum::{Json, Router, extract::{Path, Query, State}, http::StatusCode, routing::{get, post, patch, delete}};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::cors::CorsLayer;

type Db = Arc<Mutex<Connection>>;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let db = Arc::new(Mutex::new(init_db()));
    seed_db(&db).await;

    let app = Router::new()
        // Fields
        .route("/api/v1/fields", get(list_fields).post(create_field))
        .route("/api/v1/fields/{id}", get(get_field).patch(update_field).delete(delete_field))
        // Crops
        .route("/api/v1/crops", get(list_crops).post(plant_crop))
        .route("/api/v1/crops/{id}", get(get_crop))
        .route("/api/v1/crops/calendar", get(get_crop_calendar))
        .route("/api/v1/harvests", post(log_harvest).get(get_harvest_history))
        // Activities
        .route("/api/v1/activities", get(list_activities).post(log_activity))
        .route("/api/v1/activities/summary", get(get_activity_summary))
        // Intelligence
        .route("/api/v1/weather/forecast", get(get_forecast))
        .route("/api/v1/weather/historical", get(get_historical_weather))
        .route("/api/v1/weather/alerts", get(get_weather_alerts))
        .route("/api/v1/weather/gdd", get(get_gdd))
        .route("/api/v1/satellite/ndvi", get(get_ndvi))
        .route("/api/v1/satellite/health", get(get_crop_health))
        .route("/api/v1/satellite/anomalies", get(get_anomalies))
        .route("/api/v1/satellite/boundary", get(get_boundary))
        .route("/api/v1/market/price", get(get_commodity_price))
        .route("/api/v1/market/history", get(get_price_history))
        .route("/api/v1/market/trends", get(get_market_trends))
        .route("/api/v1/market/commodities", get(list_commodities))
        .route("/api/v1/market/best-sell-time", get(get_best_sell_time))
        .route("/api/v1/iot/sensors", get(list_sensors))
        .route("/api/v1/iot/sensors/{id}", get(get_sensor_reading))
        .route("/api/v1/iot/soil-moisture", get(get_soil_moisture))
        .route("/api/v1/iot/rainfall", get(get_rainfall))
        .route("/api/v1/yield/estimate", get(estimate_yield))
        .route("/api/v1/yield/compare", get(compare_seasons))
        .route("/api/v1/alerts/pests", get(get_pest_alerts))
        .route("/api/v1/alerts/disease-risk", get(get_disease_risk))
        .layer(CorsLayer::permissive())
        .with_state(db);

    tracing::info!("AgTech Backend running on http://localhost:7800");
    tracing::info!("API: http://localhost:7800/api/v1");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:7800").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn init_db() -> Connection {
    let conn = Connection::open("agtech.db").unwrap();
    conn.execute_batch("
        CREATE TABLE IF NOT EXISTS fields (
            id TEXT PRIMARY KEY, name TEXT, area_hectares REAL, lat REAL, lon REAL,
            soil_type TEXT, elevation_m INTEGER, irrigation TEXT, region TEXT, created_at TEXT
        );
        CREATE TABLE IF NOT EXISTS crops (
            id TEXT PRIMARY KEY, field_id TEXT, crop_name TEXT, variety TEXT,
            planted_at TEXT, expected_harvest TEXT, growth_stage TEXT, status TEXT DEFAULT 'growing'
        );
        CREATE TABLE IF NOT EXISTS activities (
            id TEXT PRIMARY KEY, field_id TEXT, activity_type TEXT, description TEXT,
            cost REAL DEFAULT 0, date TEXT, created_at TEXT
        );
        CREATE TABLE IF NOT EXISTS harvests (
            id TEXT PRIMARY KEY, field_id TEXT, crop_id TEXT, crop_name TEXT,
            yield_kg REAL, area_hectares REAL, quality TEXT, date TEXT, season TEXT
        );
        CREATE TABLE IF NOT EXISTS sensors (
            id TEXT PRIMARY KEY, field_id TEXT, sensor_type TEXT, name TEXT,
            lat REAL, lon REAL, status TEXT DEFAULT 'active'
        );
    ").unwrap();
    conn
}

async fn seed_db(db: &Db) {
    let conn = db.lock().await;
    let count: i64 = conn.query_row("SELECT COUNT(*) FROM fields", [], |r| r.get(0)).unwrap();
    if count > 0 { return; }

    conn.execute_batch("
        INSERT INTO fields VALUES ('f1','Kiambu Coffee Estate',12.5,-1.1714,36.8314,'red loam',1800,'drip','Kiambu','2024-01-15');
        INSERT INTO fields VALUES ('f2','Nakuru Wheat Field',45.0,-0.3031,36.0800,'black cotton',1850,'rainfed','Nakuru','2024-03-01');
        INSERT INTO fields VALUES ('f3','Meru Tea Plantation',8.2,0.0480,37.6560,'volcanic',2100,'sprinkler','Meru','2023-06-10');
        INSERT INTO fields VALUES ('f4','Machakos Maize Plot',5.0,-1.5177,37.2634,'sandy loam',1200,'rainfed','Machakos','2024-09-01');
        INSERT INTO fields VALUES ('f5','Kericho Avocado Orchard',3.5,-0.3692,35.2863,'clay loam',2000,'drip','Kericho','2023-11-20');

        INSERT INTO crops VALUES ('c1','f1','Coffee','SL28','2024-01-20','2026-10-15','fruiting','growing');
        INSERT INTO crops VALUES ('c2','f2','Wheat','Kenya Eagle','2026-03-15','2026-07-20','tillering','growing');
        INSERT INTO crops VALUES ('c3','f3','Tea','TRFK 6/8','2023-06-15',NULL,'mature','growing');
        INSERT INTO crops VALUES ('c4','f4','Maize','H614D','2026-04-01','2026-08-15','vegetative','growing');
        INSERT INTO crops VALUES ('c5','f5','Avocado','Hass','2023-12-01','2026-06-01','fruiting','growing');

        INSERT INTO activities VALUES ('a1','f1','fertilizer','Applied NPK 17-17-17 at 200kg/ha',15000,'2026-05-10','2026-05-10');
        INSERT INTO activities VALUES ('a2','f2','spraying','Fungicide application for rust prevention',8500,'2026-05-15','2026-05-15');
        INSERT INTO activities VALUES ('a3','f4','weeding','Manual weeding - 10 workers',12000,'2026-05-18','2026-05-18');
        INSERT INTO activities VALUES ('a4','f3','plucking','Tea plucking - 2 leaves and a bud',25000,'2026-05-20','2026-05-20');
        INSERT INTO activities VALUES ('a5','f1','irrigation','Drip irrigation cycle - 4 hours',3000,'2026-05-22','2026-05-22');

        INSERT INTO harvests VALUES ('h1','f1','c1','Coffee',4500,12.5,'AA','2025-11-20','2025 Long Rains');
        INSERT INTO harvests VALUES ('h2','f2','c2','Wheat',135000,45.0,'Grade 1','2025-07-25','2025 Long Rains');
        INSERT INTO harvests VALUES ('h3','f3','c3','Tea',24600,8.2,'Premium','2025-12-30','2025 Q4');
        INSERT INTO harvests VALUES ('h4','f4','c4','Maize',18000,5.0,'Grade 2','2025-08-10','2025 Long Rains');

        INSERT INTO sensors VALUES ('s1','f1','soil_moisture','Coffee Field Moisture Sensor',-1.1714,36.8314,'active');
        INSERT INTO sensors VALUES ('s2','f1','temperature','Coffee Field Temp Sensor',-1.1714,36.8314,'active');
        INSERT INTO sensors VALUES ('s3','f2','rain_gauge','Wheat Field Rain Gauge',-0.3031,36.0800,'active');
        INSERT INTO sensors VALUES ('s4','f4','soil_moisture','Maize Plot Moisture',-1.5177,37.2634,'active');
        INSERT INTO sensors VALUES ('s5','f3','humidity','Tea Plantation Humidity',0.0480,37.6560,'active');
    ").unwrap();
    tracing::info!("Database seeded with Kenyan farm data (5 fields, 5 crops, 5 activities, 4 harvests, 5 sensors)");
}

// ─── Farm Management Handlers ────────────────────────────────────────────────

#[derive(Deserialize)]
struct ListQuery { field_id: Option<String>, status: Option<String> }

async fn list_fields(State(db): State<Db>) -> Json<Vec<serde_json::Value>> {
    let conn = db.lock().await;
    let mut stmt = conn.prepare("SELECT id,name,area_hectares,lat,lon,soil_type,elevation_m,irrigation,region FROM fields").unwrap();
    let rows = stmt.query_map([], |r| Ok(serde_json::json!({"id":r.get::<_,String>(0)?,"name":r.get::<_,String>(1)?,"area_hectares":r.get::<_,f64>(2)?,"lat":r.get::<_,f64>(3)?,"lon":r.get::<_,f64>(4)?,"soil_type":r.get::<_,String>(5)?,"elevation_m":r.get::<_,i32>(6)?,"irrigation":r.get::<_,String>(7)?,"region":r.get::<_,String>(8)?}))).unwrap().filter_map(|r| r.ok()).collect();
    Json(rows)
}

async fn get_field(State(db): State<Db>, Path(id): Path<String>) -> Json<serde_json::Value> {
    let conn = db.lock().await;
    let row = conn.query_row("SELECT id,name,area_hectares,lat,lon,soil_type,elevation_m,irrigation,region FROM fields WHERE id=?", [&id], |r| Ok(serde_json::json!({"id":r.get::<_,String>(0)?,"name":r.get::<_,String>(1)?,"area_hectares":r.get::<_,f64>(2)?,"lat":r.get::<_,f64>(3)?,"lon":r.get::<_,f64>(4)?,"soil_type":r.get::<_,String>(5)?,"elevation_m":r.get::<_,i32>(6)?,"irrigation":r.get::<_,String>(7)?,"region":r.get::<_,String>(8)?}))).unwrap_or(serde_json::json!({"error":"not found"}));
    Json(row)
}

#[derive(Deserialize)]
struct CreateField { name: String, area_hectares: f64, lat: f64, lon: f64, soil_type: Option<String>, elevation_m: Option<i32>, irrigation: Option<String>, region: Option<String> }

async fn create_field(State(db): State<Db>, Json(i): Json<CreateField>) -> (StatusCode, Json<serde_json::Value>) {
    let id = format!("f-{}", &uuid::Uuid::new_v4().to_string()[..8]);
    let conn = db.lock().await;
    conn.execute("INSERT INTO fields VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)", rusqlite::params![id, i.name, i.area_hectares, i.lat, i.lon, i.soil_type.unwrap_or("unknown".into()), i.elevation_m.unwrap_or(0), i.irrigation.unwrap_or("rainfed".into()), i.region.unwrap_or("".into()), chrono::Utc::now().to_rfc3339()]).unwrap();
    (StatusCode::CREATED, Json(serde_json::json!({"id": id, "name": i.name})))
}

#[derive(Deserialize)]
struct UpdateField { name: Option<String>, soil_type: Option<String>, irrigation: Option<String> }

async fn update_field(State(db): State<Db>, Path(id): Path<String>, Json(i): Json<UpdateField>) -> Json<serde_json::Value> {
    let conn = db.lock().await;
    if let Some(n) = i.name { conn.execute("UPDATE fields SET name=? WHERE id=?", rusqlite::params![n, id]).unwrap(); }
    if let Some(s) = i.soil_type { conn.execute("UPDATE fields SET soil_type=? WHERE id=?", rusqlite::params![s, id]).unwrap(); }
    if let Some(ir) = i.irrigation { conn.execute("UPDATE fields SET irrigation=? WHERE id=?", rusqlite::params![ir, id]).unwrap(); }
    Json(serde_json::json!({"updated": true, "id": id}))
}

async fn delete_field(State(db): State<Db>, Path(id): Path<String>) -> Json<serde_json::Value> {
    let conn = db.lock().await;
    conn.execute("DELETE FROM fields WHERE id=?", [&id]).unwrap();
    Json(serde_json::json!({"deleted": true, "id": id}))
}

async fn list_crops(State(db): State<Db>, Query(q): Query<ListQuery>) -> Json<Vec<serde_json::Value>> {
    let conn = db.lock().await;
    let mut sql = "SELECT id,field_id,crop_name,variety,planted_at,expected_harvest,growth_stage,status FROM crops WHERE 1=1".to_string();
    if let Some(ref f) = q.field_id { sql.push_str(&format!(" AND field_id='{f}'")); }
    let mut stmt = conn.prepare(&sql).unwrap();
    let rows = stmt.query_map([], |r| Ok(serde_json::json!({"id":r.get::<_,String>(0)?,"field_id":r.get::<_,String>(1)?,"crop_name":r.get::<_,String>(2)?,"variety":r.get::<_,String>(3)?,"planted_at":r.get::<_,String>(4)?,"expected_harvest":r.get::<_,Option<String>>(5)?,"growth_stage":r.get::<_,String>(6)?,"status":r.get::<_,String>(7)?}))).unwrap().filter_map(|r| r.ok()).collect();
    Json(rows)
}

async fn get_crop(State(db): State<Db>, Path(id): Path<String>) -> Json<serde_json::Value> {
    let conn = db.lock().await;
    conn.query_row("SELECT id,field_id,crop_name,variety,planted_at,expected_harvest,growth_stage,status FROM crops WHERE id=?", [&id], |r| Ok(serde_json::json!({"id":r.get::<_,String>(0)?,"field_id":r.get::<_,String>(1)?,"crop_name":r.get::<_,String>(2)?,"variety":r.get::<_,String>(3)?,"planted_at":r.get::<_,String>(4)?,"expected_harvest":r.get::<_,Option<String>>(5)?,"growth_stage":r.get::<_,String>(6)?,"status":r.get::<_,String>(7)?}))).unwrap_or(serde_json::json!({"error":"not found"})).into()
}

#[derive(Deserialize)]
struct PlantCrop { field_id: String, crop_name: String, variety: Option<String>, expected_harvest: Option<String> }

async fn plant_crop(State(db): State<Db>, Json(i): Json<PlantCrop>) -> (StatusCode, Json<serde_json::Value>) {
    let id = format!("c-{}", &uuid::Uuid::new_v4().to_string()[..8]);
    let conn = db.lock().await;
    conn.execute("INSERT INTO crops VALUES (?1,?2,?3,?4,?5,?6,'seedling','growing')", rusqlite::params![id, i.field_id, i.crop_name, i.variety.unwrap_or("".into()), chrono::Utc::now().format("%Y-%m-%d").to_string(), i.expected_harvest]).unwrap();
    (StatusCode::CREATED, Json(serde_json::json!({"id": id, "crop": i.crop_name, "status": "planted"})))
}

#[derive(Deserialize)]
struct CropCalendarQuery { crop: Option<String>, region: Option<String> }

async fn get_crop_calendar(Query(q): Query<CropCalendarQuery>) -> Json<serde_json::Value> {
    let crop = q.crop.unwrap_or("maize".into());
    Json(serde_json::json!({"crop": crop, "region": q.region.unwrap_or("Central Kenya".into()), "calendar": [
        {"month": "March", "activity": "Land preparation", "notes": "Start of long rains"},
        {"month": "April", "activity": "Planting", "notes": "Plant with first rains"},
        {"month": "May-June", "activity": "Weeding & fertilizer", "notes": "Top dress at knee height"},
        {"month": "July", "activity": "Pest monitoring", "notes": "Watch for fall armyworm"},
        {"month": "August", "activity": "Harvest", "notes": "120 days after planting"}
    ]}))
}

#[derive(Deserialize)]
struct LogHarvest { field_id: String, crop_id: String, crop_name: String, yield_kg: f64, quality: Option<String> }

async fn log_harvest(State(db): State<Db>, Json(i): Json<LogHarvest>) -> (StatusCode, Json<serde_json::Value>) {
    let id = format!("h-{}", &uuid::Uuid::new_v4().to_string()[..8]);
    let conn = db.lock().await;
    let area: f64 = conn.query_row("SELECT area_hectares FROM fields WHERE id=?", [&i.field_id], |r| r.get(0)).unwrap_or(1.0);
    conn.execute("INSERT INTO harvests VALUES (?1,?2,?3,?4,?5,?6,?7,?8,'2026 Long Rains')", rusqlite::params![id, i.field_id, i.crop_id, i.crop_name, i.yield_kg, area, i.quality.unwrap_or("Standard".into()), chrono::Utc::now().format("%Y-%m-%d").to_string()]).unwrap();
    conn.execute("UPDATE crops SET status='harvested' WHERE id=?", [&i.crop_id]).unwrap();
    (StatusCode::CREATED, Json(serde_json::json!({"id": id, "yield_kg": i.yield_kg, "yield_per_ha": i.yield_kg / area})))
}

async fn get_harvest_history(State(db): State<Db>, Query(q): Query<ListQuery>) -> Json<Vec<serde_json::Value>> {
    let conn = db.lock().await;
    let mut sql = "SELECT id,field_id,crop_name,yield_kg,area_hectares,quality,date,season FROM harvests WHERE 1=1".to_string();
    if let Some(ref f) = q.field_id { sql.push_str(&format!(" AND field_id='{f}'")); }
    sql.push_str(" ORDER BY date DESC");
    let mut stmt = conn.prepare(&sql).unwrap();
    let rows = stmt.query_map([], |r| Ok(serde_json::json!({"id":r.get::<_,String>(0)?,"field_id":r.get::<_,String>(1)?,"crop_name":r.get::<_,String>(2)?,"yield_kg":r.get::<_,f64>(3)?,"area_hectares":r.get::<_,f64>(4)?,"yield_per_ha":r.get::<_,f64>(3)?/r.get::<_,f64>(4)?,"quality":r.get::<_,String>(5)?,"date":r.get::<_,String>(6)?,"season":r.get::<_,String>(7)?}))).unwrap().filter_map(|r| r.ok()).collect();
    Json(rows)
}

#[derive(Deserialize)]
struct LogActivity { field_id: String, activity_type: String, description: String, cost: Option<f64> }

async fn log_activity(State(db): State<Db>, Json(i): Json<LogActivity>) -> (StatusCode, Json<serde_json::Value>) {
    let id = format!("a-{}", &uuid::Uuid::new_v4().to_string()[..8]);
    let now = chrono::Utc::now().format("%Y-%m-%d").to_string();
    let conn = db.lock().await;
    conn.execute("INSERT INTO activities VALUES (?1,?2,?3,?4,?5,?6,?6)", rusqlite::params![id, i.field_id, i.activity_type, i.description, i.cost.unwrap_or(0.0), now]).unwrap();
    (StatusCode::CREATED, Json(serde_json::json!({"id": id, "type": i.activity_type})))
}

async fn list_activities(State(db): State<Db>, Query(q): Query<ListQuery>) -> Json<Vec<serde_json::Value>> {
    let conn = db.lock().await;
    let mut sql = "SELECT id,field_id,activity_type,description,cost,date FROM activities WHERE 1=1".to_string();
    if let Some(ref f) = q.field_id { sql.push_str(&format!(" AND field_id='{f}'")); }
    sql.push_str(" ORDER BY date DESC LIMIT 50");
    let mut stmt = conn.prepare(&sql).unwrap();
    let rows = stmt.query_map([], |r| Ok(serde_json::json!({"id":r.get::<_,String>(0)?,"field_id":r.get::<_,String>(1)?,"activity_type":r.get::<_,String>(2)?,"description":r.get::<_,String>(3)?,"cost":r.get::<_,f64>(4)?,"date":r.get::<_,String>(5)?}))).unwrap().filter_map(|r| r.ok()).collect();
    Json(rows)
}

async fn get_activity_summary(State(db): State<Db>) -> Json<serde_json::Value> {
    let conn = db.lock().await;
    let total_cost: f64 = conn.query_row("SELECT COALESCE(SUM(cost),0) FROM activities", [], |r| r.get(0)).unwrap();
    let count: i64 = conn.query_row("SELECT COUNT(*) FROM activities", [], |r| r.get(0)).unwrap();
    Json(serde_json::json!({"total_activities": count, "total_cost_kes": total_cost, "period": "all time"}))
}

// ─── Intelligence Handlers ───────────────────────────────────────────────────

#[derive(Deserialize)]
struct LocationQuery { lat: Option<f64>, lon: Option<f64>, field_id: Option<String> }
#[derive(Deserialize)]
struct CommodityQuery { commodity: Option<String>, period: Option<String> }

// Weather
async fn get_forecast(Query(q): Query<LocationQuery>) -> Json<serde_json::Value> {
    let lat = q.lat.unwrap_or(-1.17); let lon = q.lon.unwrap_or(36.83);
    Json(serde_json::json!({"location": {"lat": lat, "lon": lon}, "forecast": [
        {"date": "2026-05-26", "temp_min": 14, "temp_max": 26, "humidity": 65, "rain_mm": 2.5, "condition": "Partly cloudy"},
        {"date": "2026-05-27", "temp_min": 13, "temp_max": 24, "humidity": 78, "rain_mm": 12.0, "condition": "Rain"},
        {"date": "2026-05-28", "temp_min": 12, "temp_max": 22, "humidity": 82, "rain_mm": 18.5, "condition": "Heavy rain"},
        {"date": "2026-05-29", "temp_min": 14, "temp_max": 25, "humidity": 70, "rain_mm": 5.0, "condition": "Light rain"},
        {"date": "2026-05-30", "temp_min": 15, "temp_max": 27, "humidity": 55, "rain_mm": 0.0, "condition": "Sunny"},
        {"date": "2026-05-31", "temp_min": 14, "temp_max": 26, "humidity": 60, "rain_mm": 0.0, "condition": "Clear"},
        {"date": "2026-06-01", "temp_min": 13, "temp_max": 25, "humidity": 62, "rain_mm": 1.0, "condition": "Partly cloudy"}
    ]}))
}

async fn get_historical_weather(Query(q): Query<LocationQuery>) -> Json<serde_json::Value> {
    Json(serde_json::json!({"location": {"lat": q.lat.unwrap_or(-1.17), "lon": q.lon.unwrap_or(36.83)}, "period": "2026-04-01 to 2026-05-25", "summary": {"avg_temp": 20.5, "total_rain_mm": 245.0, "rain_days": 18, "avg_humidity": 72}, "monthly": [
        {"month": "April 2026", "avg_temp": 21.2, "rain_mm": 180.0, "rain_days": 14},
        {"month": "May 2026", "avg_temp": 19.8, "rain_mm": 65.0, "rain_days": 4}
    ]}))
}

async fn get_weather_alerts() -> Json<serde_json::Value> {
    Json(serde_json::json!({"alerts": [
        {"type": "heavy_rain", "severity": "moderate", "message": "Heavy rainfall expected May 27-28. Risk of waterlogging in low-lying fields.", "affected_regions": ["Kiambu", "Murang'a"]},
        {"type": "frost_risk", "severity": "low", "message": "Possible frost in highland areas above 2500m on May 30.", "affected_regions": ["Nyandarua", "Laikipia"]}
    ]}))
}

async fn get_gdd(Query(q): Query<CommodityQuery>) -> Json<serde_json::Value> {
    let crop = q.commodity.unwrap_or("maize".into());
    Json(serde_json::json!({"crop": crop, "base_temp_c": 10, "accumulated_gdd": 850, "required_gdd": 1400, "progress_percent": 60.7, "estimated_days_remaining": 45, "note": "On track for August harvest"}))
}

// Satellite
async fn get_ndvi(Query(q): Query<LocationQuery>) -> Json<serde_json::Value> {
    Json(serde_json::json!({"field_id": q.field_id.unwrap_or("f1".into()), "date": "2026-05-23", "ndvi": 0.72, "interpretation": "Healthy vegetation", "scale": {"0.0-0.2": "Bare soil/water", "0.2-0.4": "Sparse vegetation", "0.4-0.6": "Moderate vegetation", "0.6-0.8": "Dense healthy vegetation", "0.8-1.0": "Very dense vegetation"}, "trend": "stable", "previous_reading": {"date": "2026-05-16", "ndvi": 0.70}}))
}

async fn get_crop_health(Query(q): Query<LocationQuery>) -> Json<serde_json::Value> {
    Json(serde_json::json!({"field_id": q.field_id.unwrap_or("f1".into()), "date": "2026-05-23", "overall_health": "good", "score": 78, "indicators": {"ndvi": 0.72, "chlorophyll": "normal", "water_stress": "low", "canopy_cover_pct": 85}, "recommendations": ["Continue current irrigation schedule", "Monitor for coffee berry disease as rains continue"]}))
}

async fn get_anomalies(Query(q): Query<LocationQuery>) -> Json<serde_json::Value> {
    Json(serde_json::json!({"field_id": q.field_id.unwrap_or("f1".into()), "date": "2026-05-23", "anomalies": [
        {"type": "water_stress", "severity": "low", "area_pct": 8, "location": "northeast corner", "recommendation": "Check drip line for blockage"},
        {"type": "nutrient_deficiency", "severity": "moderate", "area_pct": 12, "location": "southern section", "recommendation": "Soil test recommended — possible nitrogen deficiency"}
    ]}))
}

async fn get_boundary(Query(q): Query<LocationQuery>) -> Json<serde_json::Value> {
    Json(serde_json::json!({"field_id": q.field_id.unwrap_or("f1".into()), "area_hectares": 12.5, "perimeter_m": 1420, "polygon": [[-1.170,36.830],[-1.170,36.835],[-1.174,36.835],[-1.174,36.830],[-1.170,36.830]]}))
}

// Market
async fn get_commodity_price(Query(q): Query<CommodityQuery>) -> Json<serde_json::Value> {
    let commodity = q.commodity.unwrap_or("coffee".into());
    let (price, unit, exchange) = match commodity.as_str() {
        "coffee" => (850.0, "KES/kg", "Nairobi Coffee Exchange"),
        "tea" => (320.0, "KES/kg", "Mombasa Tea Auction"),
        "maize" => (55.0, "KES/kg", "NCPB"),
        "wheat" => (65.0, "KES/kg", "NCPB"),
        "avocado" => (120.0, "KES/kg", "Nairobi Market"),
        _ => (0.0, "KES/kg", "Unknown"),
    };
    Json(serde_json::json!({"commodity": commodity, "price": price, "unit": unit, "exchange": exchange, "date": "2026-05-25", "change_pct": 2.3, "direction": "up"}))
}

async fn get_price_history(Query(q): Query<CommodityQuery>) -> Json<serde_json::Value> {
    let commodity = q.commodity.unwrap_or("coffee".into());
    Json(serde_json::json!({"commodity": commodity, "period": "6 months", "history": [
        {"month": "Dec 2025", "price": 780}, {"month": "Jan 2026", "price": 795},
        {"month": "Feb 2026", "price": 810}, {"month": "Mar 2026", "price": 825},
        {"month": "Apr 2026", "price": 840}, {"month": "May 2026", "price": 850}
    ], "trend": "upward", "avg_price": 817}))
}

async fn get_market_trends(Query(q): Query<CommodityQuery>) -> Json<serde_json::Value> {
    Json(serde_json::json!({"commodity": q.commodity.unwrap_or("coffee".into()), "trends": {"short_term": "bullish", "medium_term": "stable", "long_term": "bullish"}, "factors": ["Global supply shortage", "Increased demand from Asia", "Favorable exchange rate KES/USD"], "forecast": {"next_month": 870, "next_quarter": 900}}))
}

async fn list_commodities() -> Json<serde_json::Value> {
    Json(serde_json::json!({"commodities": [
        {"name": "coffee", "price": 850, "unit": "KES/kg", "change_pct": 2.3},
        {"name": "tea", "price": 320, "unit": "KES/kg", "change_pct": -1.1},
        {"name": "maize", "price": 55, "unit": "KES/kg", "change_pct": 5.0},
        {"name": "wheat", "price": 65, "unit": "KES/kg", "change_pct": 0.8},
        {"name": "avocado", "price": 120, "unit": "KES/kg", "change_pct": 8.5},
        {"name": "sugarcane", "price": 4500, "unit": "KES/ton", "change_pct": 1.2},
        {"name": "flowers", "price": 45, "unit": "KES/stem", "change_pct": -3.0}
    ]}))
}

async fn get_best_sell_time(Query(q): Query<CommodityQuery>) -> Json<serde_json::Value> {
    Json(serde_json::json!({"commodity": q.commodity.unwrap_or("coffee".into()), "recommendation": "Hold for 2-3 weeks", "reason": "Prices trending upward. Global supply tightening expected in June.", "current_price": 850, "predicted_peak": 900, "predicted_peak_date": "2026-06-15", "storage_cost_per_week": 5.0, "net_gain_if_hold": 40}))
}

// IoT
async fn list_sensors(State(db): State<Db>) -> Json<Vec<serde_json::Value>> {
    let conn = db.lock().await;
    let mut stmt = conn.prepare("SELECT id,field_id,sensor_type,name,status FROM sensors").unwrap();
    let rows = stmt.query_map([], |r| Ok(serde_json::json!({"id":r.get::<_,String>(0)?,"field_id":r.get::<_,String>(1)?,"sensor_type":r.get::<_,String>(2)?,"name":r.get::<_,String>(3)?,"status":r.get::<_,String>(4)?}))).unwrap().filter_map(|r| r.ok()).collect();
    Json(rows)
}

async fn get_sensor_reading(Path(id): Path<String>) -> Json<serde_json::Value> {
    let reading = match id.as_str() {
        "s1" => serde_json::json!({"sensor_id":"s1","type":"soil_moisture","value":42.5,"unit":"%","timestamp":"2026-05-25T14:30:00Z","status":"normal","threshold":{"low":25,"high":60}}),
        "s2" => serde_json::json!({"sensor_id":"s2","type":"temperature","value":22.3,"unit":"°C","timestamp":"2026-05-25T14:30:00Z","status":"normal"}),
        "s3" => serde_json::json!({"sensor_id":"s3","type":"rain_gauge","value":3.2,"unit":"mm/hr","timestamp":"2026-05-25T14:30:00Z","status":"raining"}),
        "s4" => serde_json::json!({"sensor_id":"s4","type":"soil_moisture","value":28.1,"unit":"%","timestamp":"2026-05-25T14:30:00Z","status":"low","alert":"Irrigation recommended"}),
        "s5" => serde_json::json!({"sensor_id":"s5","type":"humidity","value":78.0,"unit":"%","timestamp":"2026-05-25T14:30:00Z","status":"high"}),
        _ => serde_json::json!({"error":"sensor not found"}),
    };
    Json(reading)
}

async fn get_soil_moisture(Query(q): Query<LocationQuery>) -> Json<serde_json::Value> {
    Json(serde_json::json!({"field_id": q.field_id.unwrap_or("f1".into()), "readings": [
        {"depth_cm": 10, "moisture_pct": 45.2, "status": "adequate"},
        {"depth_cm": 30, "moisture_pct": 38.7, "status": "adequate"},
        {"depth_cm": 60, "moisture_pct": 32.1, "status": "slightly low"}
    ], "recommendation": "No irrigation needed for next 48 hours based on forecast rain"}))
}

async fn get_rainfall(Query(q): Query<LocationQuery>) -> Json<serde_json::Value> {
    Json(serde_json::json!({"field_id": q.field_id.unwrap_or("f1".into()), "period": "last 30 days", "total_mm": 145.0, "days_with_rain": 12, "daily": [
        {"date": "2026-05-25", "mm": 0.0}, {"date": "2026-05-24", "mm": 3.2},
        {"date": "2026-05-23", "mm": 0.0}, {"date": "2026-05-22", "mm": 8.5},
        {"date": "2026-05-21", "mm": 15.0}, {"date": "2026-05-20", "mm": 22.0}
    ], "season_total_mm": 420, "season_average_mm": 550, "deficit_mm": 130}))
}

// Yield & Alerts
async fn estimate_yield(Query(q): Query<LocationQuery>) -> Json<serde_json::Value> {
    Json(serde_json::json!({"field_id": q.field_id.unwrap_or("f1".into()), "crop": "Coffee", "estimated_yield_kg": 5200, "yield_per_ha": 416, "confidence": "medium", "factors": {"weather_score": 75, "ndvi_score": 82, "soil_moisture_score": 70, "historical_avg_kg": 4500}, "comparison": {"vs_last_season": "+15.6%", "vs_5yr_avg": "+8.3%"}}))
}

async fn compare_seasons(Query(q): Query<LocationQuery>) -> Json<serde_json::Value> {
    Json(serde_json::json!({"field_id": q.field_id.unwrap_or("f1".into()), "crop": "Coffee", "seasons": [
        {"season": "2026 (current)", "yield_kg": null, "estimated_kg": 5200, "rain_mm": 420, "status": "in progress"},
        {"season": "2025 Long Rains", "yield_kg": 4500, "rain_mm": 510, "quality": "AA", "status": "completed"},
        {"season": "2024 Long Rains", "yield_kg": 4200, "rain_mm": 480, "quality": "AB", "status": "completed"},
        {"season": "2023 Long Rains", "yield_kg": 3800, "rain_mm": 390, "quality": "AA", "status": "completed"}
    ], "trend": "improving", "avg_yield_kg": 4167}))
}

async fn get_pest_alerts() -> Json<serde_json::Value> {
    Json(serde_json::json!({"alerts": [
        {"pest": "Coffee Berry Borer", "risk": "high", "affected_crops": ["coffee"], "regions": ["Kiambu", "Nyeri", "Meru"], "recommendation": "Apply Beauveria bassiana biological control. Monitor berry infestation levels."},
        {"pest": "Fall Armyworm", "risk": "moderate", "affected_crops": ["maize"], "regions": ["Machakos", "Kitui", "Makueni"], "recommendation": "Scout fields early morning. Apply Bt-based pesticide if >5% infestation."},
        {"pest": "Aphids", "risk": "low", "affected_crops": ["wheat", "vegetables"], "regions": ["Nakuru", "Narok"], "recommendation": "Monitor. Natural predators (ladybugs) usually sufficient."}
    ], "last_updated": "2026-05-25"}))
}

async fn get_disease_risk(Query(q): Query<LocationQuery>) -> Json<serde_json::Value> {
    Json(serde_json::json!({"field_id": q.field_id.unwrap_or("f1".into()), "crop": "Coffee", "risks": [
        {"disease": "Coffee Berry Disease (CBD)", "risk_level": "high", "probability_pct": 72, "trigger": "High humidity (>80%) + rain expected next 3 days", "prevention": "Apply copper-based fungicide before rain"},
        {"disease": "Coffee Leaf Rust", "risk_level": "moderate", "probability_pct": 45, "trigger": "Warm temperatures + wet conditions", "prevention": "Ensure good air circulation, prune lower branches"},
        {"disease": "Root Rot", "risk_level": "low", "probability_pct": 15, "trigger": "Waterlogging", "prevention": "Ensure drainage channels are clear"}
    ], "overall_risk": "moderate-high", "action_required": true}))
}
