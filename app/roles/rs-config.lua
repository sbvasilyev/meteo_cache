local storage = require("app.src.storage")

local M = {}

M.role_name = "app.roles.rs-config"

function M.init(opts)
    box.schema.func.create("libmeteo.update_config", { language = "C", if_not_exists = true })

    return true
end

function M.stop()
    return true
end

function M.validate_config(_conf_new, _conf_old)
    return true
end

function M.apply_config(conf, opts)
    local ttl = conf["ttl"] or 60
    local geocode_url = conf["geocode_url"] or "http://geocoding-api.open-meteo.com/v1/search?count=1&language=en&name="
    local forecast_url = conf["forecast_url"] or "http://api.open-meteo.com/v1/forecast?"

    box.func["libmeteo.update_config"]:call({ ttl, geocode_url, forecast_url })

    return true
end

return M
