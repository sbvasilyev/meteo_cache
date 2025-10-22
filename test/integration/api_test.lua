local t = require("luatest")
local g = t.group("integration_api")
local fiber = require("fiber")

local helper = require("test.helper")

g.before_all(function(cg)
    cg.cluster = helper.cluster
    cg.cluster:start()
end)

g.after_all(function(cg)
    helper.stop_cluster(cg.cluster)
end)

g.before_each(function(cg) -- luacheck: no unused args
    helper.truncate_space_on_cluster(g.cluster, "meteo")
end)

g.test_cache_hit = function(cg)
    local server = cg.cluster.main_server
    local response =
        server:http_request("get", "/forecast?city=Moscow&timezone=auto&forecast_days=3&hourly=temperature_2m")
    t.assert_equals(response.status, 200)

    local repeat_response =
        server:http_request("get", "/forecast?city=Moscow&timezone=auto&forecast_days=3&hourly=temperature_2m")
    t.assert_equals(repeat_response.status, 200)
    t.assert_equals(response.body, repeat_response.body)
end

g.test_set_ttl_in_config = function(cg)
    local config = {
        ttl = 42,
    }
    cg.cluster:upload_config(config)
    local derived_config = cg.cluster:download_config()
    t.assert_equals(derived_config.ttl, 42)
end

g.test_cache_miss = function(cg)
    local config = {
        ttl = 0,
    }
    cg.cluster:upload_config(config)

    local server = cg.cluster.main_server
    local response =
        server:http_request("get", "/forecast?city=Moscow&timezone=auto&forecast_days=3&hourly=temperature_2m")

    local start_time = fiber.time()
    local repeat_response

    while fiber.time() - start_time < 5 do
        fiber.sleep(1)
        repeat_response =
            server:http_request("get", "/forecast?city=Moscow&timezone=auto&forecast_days=3&hourly=temperature_2m")

        if repeat_response.status == 200 and repeat_response.body ~= response.body then
            break
        end
    end

    t.assert_not_equals(response.body, repeat_response.body)
end
