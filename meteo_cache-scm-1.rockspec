package = "meteo_cache"
version = "scm-1"
source = {
	url = "/dev/null",
}
-- Put any modules your app depends on here
dependencies = {
	"tarantool",
	"lua >= 5.1",
	"checks == 3.3.0-1",
	"cartridge == 2.11.0-1",
	"metrics == 1.0.0-1",
	"cartridge-metrics-role == 0.1.1-1",
	"cartridge-cli-extensions == 1.1.1-1",
	"expirationd == 1.7.0",
}
build = {
	type = "none",
}
