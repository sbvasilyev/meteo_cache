local rust = require("app.rust")

local M = {}

M.role_name = "app.roles.rs-router"
M.dependencies = { "cartridge.roles.vshard-router", "app.roles.rs-config" }

function M.init(_opts)
    rust.init("router")
    rust.http_init()

    return true
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
