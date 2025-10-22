local cartridge = require("cartridge")

local M = {}

M.role_name = "app.roles.expirator"
M.dependencies = { "cartridge.roles.expirationd" }

local function is_expired(args, tuple)
    return tuple.expires_at <= os.time()
end

local function delete_tuple(space_id, args, tuple)
    box.space[space_id]:delete({ tuple.key })
end

local function start_expirator()
    local expirationd = assert(cartridge.service_get("expirationd"), "Failed to get expirationd service")
    expirationd.start("clear_meteo_expired", box.space.meteo.id, is_expired, {
        process_expired_tuple = delete_tuple,
        tuples_per_iteration = 50,
        full_scan_time = 30,
    })
end

function M.init(opts)
    return true
end

function M.stop()
    return true
end

function M.validate_config(conf_new, conf_old)
    return true
end

function M.apply_config(conf, opts)
    if opts.is_master then
        start_expirator()
    end

    return true
end

return M
