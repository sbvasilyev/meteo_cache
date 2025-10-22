local M = {}

---Derives city name and persistent in order of keys query params string,
---given http.server.Request object
---@param req any
---@return string city "Name of the city"
---@return string query_string "Query string with ordered keys"
---@return string? error "Error message (in case of the error)"
function M.parse_query(req)
    local query = req:query_param()

    if not query.city then
        return "", "", "city param is not provided"
    end

    local city = query.city
    query.city = nil

    -- sort to persist order in query string
    local t = {}
    for k, v in pairs(query) do
        table.insert(t, { k, v })
    end
    table.sort(t, function(a, b)
        return a[1] < b[1] or (a[1] == b[1] and a[2] < b[2])
    end)
    local parts = {}
    for _, kv in ipairs(t) do
        table.insert(parts, kv[1] .. "=" .. kv[2])
    end
    return city, table.concat(parts, "&"), nil
end

return M
