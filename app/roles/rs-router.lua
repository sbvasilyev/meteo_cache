local cartridge = require("cartridge")
local handlers = require("app.src.handlers")

local M = {}

M.role_name = "app.roles.rs-router"
M.dependencies = { "cartridge.roles.vshard-router", "app.roles.rs-config" }

function M.init(_opts)
    require("libmeteo")

    box.schema.func.create("libmeteo.http_init", { language = "C", if_not_exists = true })
    box.func["libmeteo.http_init"]:call({})

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
