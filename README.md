# Booth2RSS

A tool for creating RSS feeds from Booth.pm store pages.

## Usage

### CLI
```bash
Booth2RSS.CLI <store-url> (--unblur-nsfw --allow-nsfw --include-unavailable --vrc-only --max-pages=<max-pages> --convert-currency=<abbreviated-currency-name>)
```

### Web

Run `Booth2RSS.Web`, then access the RSS feed via:
https://localhost:8080/booth2rss/store?url=<store-url>

Optional arguments:
* `max-pages=<number>`: Maximum number of pages to fetch
* `allow_nsfw=true|false`: Enable fetching of NSFW content.
* `unblur_nsfw=true|false`: Disable the 18+ placeholder image on NSFW items to show the actual thumbnail.
* `vrc_only=true|false`: Only show VRChat items
* `filter_unavailable=true|false`: Hide out-of-stock or discontinued items from the feed. (Defaults to `true`)
* `currency=<short-name>`: Try to convert the item price into the given currency (Accepts three-letter short forms, e.g. "EUR")


## Configuration (Web only)

### Standalone
A default configuration can be found in `config.json.example`.
To modify the configuration, copy the file to `config.json`.

### Docker
The docker container uses the following environment variables for configuration:
* `RUST_LOG`: The log level for the application (default: `Info`)
* `LISTEN_ADDRESS`: The address the server should listen on (default: `0.0.0.0:8080`)
* `CURRENCY_FALLBACK`: The fallback source currency for currency conversions (default: `JPY`)
* `STORE_CACHE_MINUTES`: TTL of cached stores (default: `15`)
* `STORE_CACHE_SIZE`: How many stores can be cached at the same time (default: `50`)
* `ALLOW_CURRENCY_CONVERSION`: Enables currency conversion via the `currency` URL parameter (default: `true`)
* `CURRENCY_CACHE_MINUTES`: TTL of cached currency conversion rates (default: `120`)
* `CURRENCY_CACHE_SIZE`: Maximum number of cached currency conversion rates (default: `10`)
