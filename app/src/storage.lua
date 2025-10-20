local cartridge = require("cartridge")

local M = {}

---Adds provided forecast data to the meteo cache.
---@param request_key string
---@param request_body string
---@param bucket_id number
---@return boolean
function M.put_value(request_key, request_body, bucket_id)
    local ttl = rawget(_G, "ttl") or 60
    local exp_epoch = ttl + os.time()

    box.begin()
    box.space.meteo:insert({
        request_key,
        bucket_id,
        request_body,
        exp_epoch,
    })
    box.commit()

    return true
end

---Get cached forecast data if exists (otherwise nil).
---@param request_key string
---@return string?
function M.get_value(request_key)
    local entry = box.space.meteo:get(request_key)

    if entry == nil then
        return nil
    end

    return entry.value
end

return M
