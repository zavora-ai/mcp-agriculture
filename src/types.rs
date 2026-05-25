use rmcp::schemars;
use serde::Deserialize;

// ─── Fields ──────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct GetFieldInput {
    /// Field ID
    pub id: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct CreateFieldInput {
    pub name: String,
    pub area_hectares: f64,
    pub lat: f64,
    pub lon: f64,
    #[serde(default)]
    pub soil_type: Option<String>,
    #[serde(default)]
    pub elevation_m: Option<i32>,
    #[serde(default)]
    pub irrigation: Option<String>,
    #[serde(default)]
    pub region: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct UpdateFieldInput {
    pub id: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub soil_type: Option<String>,
    #[serde(default)]
    pub irrigation: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct DeleteFieldInput {
    pub id: String,
}

// ─── Crops ───────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ListCropsInput {
    #[serde(default)]
    pub field_id: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct GetCropInput {
    pub id: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct PlantCropInput {
    pub field_id: String,
    pub crop_name: String,
    #[serde(default)]
    pub variety: Option<String>,
    #[serde(default)]
    pub expected_harvest: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct CropCalendarInput {
    #[serde(default)]
    pub crop: Option<String>,
    #[serde(default)]
    pub region: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct LogHarvestInput {
    pub field_id: String,
    pub crop_id: String,
    pub crop_name: String,
    pub yield_kg: f64,
    #[serde(default)]
    pub quality: Option<String>,
}

// ─── Activities ──────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct LogActivityInput {
    pub field_id: String,
    pub activity_type: String,
    pub description: String,
    #[serde(default)]
    pub cost: Option<f64>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ListActivitiesInput {
    #[serde(default)]
    pub field_id: Option<String>,
}

// ─── Weather ─────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct LocationInput {
    #[serde(default)]
    pub lat: Option<f64>,
    #[serde(default)]
    pub lon: Option<f64>,
    #[serde(default)]
    pub field_id: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct GddInput {
    #[serde(default)]
    pub commodity: Option<String>,
}

// ─── Market ──────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct CommodityInput {
    #[serde(default)]
    pub commodity: Option<String>,
    #[serde(default)]
    pub period: Option<String>,
}

// ─── IoT ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct SensorInput {
    pub id: String,
}
