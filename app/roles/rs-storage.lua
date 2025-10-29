local storage = require("app.src.storage")

local M = {}

M.role_name = "app.roles.rs-storage"
M.dependencies = { "cartridge.roles.vshard-storage", "app.roles.storage" }

function M.init(opts)
    require("libmeteo")

    box.schema.func.create("libmeteo.rpc_init", { language = "C", if_not_exists = true })
    box.func["libmeteo.rpc_init"]:call({})

    box.schema.func.create("libmeteo.storage_rpc_handler", { language = "C", if_not_exists = true })
    rawset(_G, "rpc_handler", function(path, ctx, mp_request)
        return box.func["libmeteo.storage_rpc_handler"]:call({ path, ctx, mp_request })
    end)

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
