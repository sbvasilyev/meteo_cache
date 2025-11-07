local M = {}

local funcs_to_load = {
    ["router"] = {
        "http_init",
    },
    ["storage"] = {
        "rpc_init",
        "storage_rpc_handler",
        "init_meteo_space",
    },
    ["config"] = {
        "update_config",
    },
}

function M.init(role)
    local meteo = box.lib.load("libmeteo")

    for _, func_name in ipairs(funcs_to_load[role]) do
        local func = meteo:load(func_name)
        M[func_name] = function(...)
            local err, res = pcall(func, ...)
        end
    end
end

return M
