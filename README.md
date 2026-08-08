# Agriculture MCP Server

[![Crates.io](https://img.shields.io/crates/v/mcp-agriculture.svg)](https://crates.io/crates/mcp-agriculture)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
[![ADK-Rust Enterprise](https://img.shields.io/badge/ADK--Rust-Enterprise-purple.svg)](https://enterprise.adk-rust.com)

Full-stack agriculture MCP server with **35 tools** — farm management, weather intelligence, satellite imagery, commodity markets, IoT sensors, and yield forecasting. Includes a built-in example AgTech backend with realistic Kenyan farm data.

## Quick Start

```bash
# 1. Start the example backend (seeded with Kenyan farm data)
cd example-backend && cargo run &

# 2. Run the MCP server (connects to localhost:7800)
cd .. && cargo run
```

No API keys needed — the example backend provides all intelligence data locally.

## Tools (35)

| Category | Tools | Description |
|----------|-------|-------------|
| **Fields** (5) | list, get, create, update, delete | Farm plots with GPS, soil type, elevation |
| **Crops** (5) | list, get, plant, calendar, log_harvest | Crop lifecycle from planting to harvest |
| **Activities** (3) | log, list, summary | Farm operations (irrigation, fertilizer, spraying) |
| **Weather** (4) | forecast, historical, alerts, GDD | 7-day forecast, growing degree days |
| **Satellite** (4) | NDVI, crop_health, anomalies, boundary | Remote sensing crop assessment |
| **Market** (5) | price, history, trends, commodities, best_sell_time | Commodity prices and sell recommendations |
| **IoT** (4) | sensors, reading, soil_moisture, rainfall | Real-time sensor data |
| **Yield/Alerts** (5) | estimate, harvest_history, compare_seasons, pests, disease_risk | Intelligence and alerts |

## Example Backend

The `example-backend/` is a standalone Axum server with SQLite, seeded with:

- **5 fields:** Kiambu Coffee Estate (12.5ha), Nakuru Wheat (45ha), Meru Tea (8.2ha), Machakos Maize (5ha), Kericho Avocado (3.5ha)
- **5 crops:** SL28 Coffee, Kenya Eagle Wheat, TRFK 6/8 Tea, H614D Maize, Hass Avocado
- **5 IoT sensors:** Soil moisture, temperature, rain gauge, humidity
- **Intelligence:** Weather forecasts, NDVI readings, commodity prices (KES), pest alerts, disease risk

All data is realistic for Central/Western Kenya growing conditions.

## Configuration

### With example backend (zero config)
```bash
export AGTECH_API_URL=http://localhost:7800/api/v1
```

### With external AgTech platform
```bash
export AGTECH_API_URL=https://your-platform.com/api/v1
export AGTECH_API_KEY=your-key
```

## Client Configuration

```json
{
  "mcpServers": {
    "agriculture": {
      "command": "mcp-agriculture",
      "args": [],
      "env": {
        "AGTECH_API_URL": "http://localhost:7800/api/v1"
      }
    }
  }
}
```

## Usage Examples

```
"List all my farm fields"
→ list_fields()

"What's the weather forecast for my coffee field?"
→ get_forecast(field_id: "f1")

"How healthy is my coffee crop?"
→ get_crop_health(field_id: "f1")

"What's the current coffee price?"
→ get_commodity_price(commodity: "coffee")

"Should I sell my coffee now or wait?"
→ get_best_sell_time(commodity: "coffee")

"What's the soil moisture in my maize plot?"
→ get_soil_moisture(field_id: "f4")

"Any pest alerts for my region?"
→ get_pest_alerts()

"Estimate my coffee yield this season"
→ estimate_yield(field_id: "f1")
```

## Registry Compliance

- **HealthCheck** — verifies AgTech backend connectivity
- **mcp-server.toml** — 35 tools with risk classes
- **Manifest validation** — startup fails fast on invalid manifest

## License

Apache-2.0

---

Part of the [ADK-Rust Enterprise](https://enterprise.adk-rust.com) MCP server ecosystem.

## rmcp and MCP compatibility

This server is built with [`rmcp` 3.1.2](https://github.com/modelcontextprotocol/rust-sdk/releases/tag/rmcp-v3.1.2) and requires Rust 1.88 or newer. The rmcp 3 rollout retains legacy MCP initialization compatibility and targets MCP protocol revisions `2025-11-25` and `2026-07-28`.
