local cartridge = require("cartridge")
local handlers = require("app.src.handlers")

local M = {}

M.role_name = "app.roles.router"
M.dependencies = { "cartridge.roles.vshard-router" }

function M.init(_opts)
    local httpd = assert(cartridge.service_get("httpd"), "Failed to get httpd service")
    httpd:route({ path = "/hc", method = "GET" }, handlers.health_check)
    httpd:route({ path = "/forecast", method = "GET" }, handlers.forecast)
end

function M.stop()
    return true
end

function M.validate_config(_conf_new, _conf_old)
    return true
end

function M.apply_config(conf, opts)
    return true
end

return M
