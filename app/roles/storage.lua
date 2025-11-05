local storage = require("app.src.storage")

local M = {}

M.role_name = "app.roles.storage"
M.dependencies = { "cartridge.roles.vshard-storage", "app.roles.rs-storage" }

function M.init(opts)
    rawset(_G, "put_value", storage.put_value)
    rawset(_G, "get_value", storage.get_value)

    return true
end

function M.stop()
    return true
end

function M.validate_config(_conf_new, _conf_old)
    return true
end

function M.apply_config(conf, opts)
    if conf["ttl"] then
        rawset(_G, "ttl", tonumber(conf["ttl"]))
    end

    return true
end

return M
