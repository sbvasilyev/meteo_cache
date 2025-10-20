local storage = require("app.src.storage")

local M = {}

M.role_name = "app.roles.storage"
M.dependencies = { "cartridge.roles.vshard-storage" }

function M.init(opts)
    rawset(_G, "put_value", storage.put_value)
    rawset(_G, "get_value", storage.get_value)

    if opts.is_master then
        local meteo = box.schema.space.create("meteo", { if_not_exists = true })

        meteo:format({
            { "key", "string" },
            { "bucket_id", "unsigned" },
            { "value", "string" },
            { "expires_at", "unsigned" },
        })

        meteo:create_index("primary", { parts = { "key" }, if_not_exists = true })
    end

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
