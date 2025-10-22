local http_client = require("http.client").new({ max_connections = 100, timeout = 5 })
local json = require("json")

local M = {}

local geocode_url = "https://geocoding-api.open-meteo.com/v1/search?count=1&language=en&name=%s"
local forecast_url = "https://api.open-meteo.com/v1/forecast?latitude=%s&longitude=%s&%s"

---Returns coordinates for the provided city using Open-Meteo Geocoding API.
---@param city string
---@return string lat "Latitude of the provided city"
---@return string lon "Longitude of the provided city"
---@return string? error "Error message (in case of error)"
function M.get_city_coords(city)
    local resp = http_client:get(geocode_url:format(city))
    if not resp or resp.status ~= 200 then
        return "", "", "geocoding_request_error"
    end

    local resp_body = json.decode(resp.body)

    -- even with `?count=1` OpenMeteo API returns results as list
    local location_data = resp_body and resp_body.results and resp_body.results[1]
    if not location_data then
        return "", "", "geocoding_not_found"
    end

    return tostring(location_data.latitude), tostring(location_data.longitude), nil
end

---Returns response body as a string for the request to the Open-Meteo Forecast API
---given coordinates and provided query params.
---@param lat string "Latitude"
---@param lon string "Longitude"
---@param query_params string "Query params string (is appended to the request url)"
---@return string "Response body"
---@return string? "Error message (in case of error)"
function M.fetch_forecast(lat, lon, query_params)
    local resp = http_client:get(forecast_url:format(lat, lon, query_params))
    if not resp or resp.status ~= 200 then
        return "", "forecast_request_error"
    end

    return resp.body, nil
end

return M
