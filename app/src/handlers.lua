local vshard = require("vshard")
local fiber = require("fiber")
local log = require("log")
local helpers = require("app.src.helpers")
local weather_service = require("app.src.weather_service")

local M = {}

function M.health_check()
    log.info("health check called")

    return { status = 200, body = "alive" }
end

function M.forecast(req)
    local city, query_string, request_err = helpers.parse_query(req)
    if request_err then
        return { status = 400, body = request_err }
    end

    local lat, lon, coords_err = weather_service.get_city_coords(city)
    if coords_err then
        return { status = 400, body = coords_err }
    end

    local key = city .. "||" .. query_string
    local bucket_id = vshard.router.bucket_id_strcrc32(key)

    local read_body = vshard.router.call(bucket_id, "read", "get_value", { key }, {})
    if read_body then
        return { status = 200, body = read_body }
    end

    local fcast_body, fcast_err = weather_service.fetch_forecast(lat, lon, query_string)
    if fcast_err then
        return { status = 400, body = coords_err }
    end

    fiber.create(function()
        if not vshard.router.call(bucket_id, "write", "put_value", { key, fcast_body, bucket_id }, {}) then
            log.warn("unable to write data: " .. key .. " :: " .. fcast_body)
        end
    end)

    return { status = 200, body = fcast_body }
end

return M
