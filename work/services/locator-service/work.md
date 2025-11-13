# Coordinates for Tunis, Tunisia (city center)
LONGITUDE=10.1815
LATITUDE=36.8065

# Get nearby stations in Tunis (basic)
curl "http://localhost:8080/api/stations/nearby?longitude=10.1815&latitude=36.8065&radius_km=20&limit=10"

# Get nearby stations with custom radius (10km around Tunis)
curl "http://localhost:8080/api/stations/nearby?longitude=10.1815&latitude=36.8065&radius_km=10&limit=20&offset=0"

# Get detailed nearby stations with filtering in Tunis
curl "http://localhost:8080/api/stations/nearby?longitude=10.1815&latitude=36.8065&radius_km=10&limit=10"

# Get detailed with connector type filtering in Tunis
curl "http://localhost:8080/api/stations/nearby/detailed?longitude=10.1815&latitude=36.8065&connector_types=type2,ccs&power_tiers=medium,fast"

# Get detailed with all filters in Tunis
curl "http://localhost:8080/api/stations/nearby/detailed?longitude=10.1815&latitude=36.8065&radius_km=15&min_power_kw=50&connector_types=ccs&power_tiers=fast,ultra_fast&limit=10&offset=0"