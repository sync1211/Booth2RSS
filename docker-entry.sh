#!/bin/sh

cat >config.json <<EOL
{
    "bind_address": "${BIND_TO:=0.0.0.0:8080}",
    "currency_fallback": "${CURRENCY_FALLBACK:=JPY}",
    "cache_minutes": ${STORE_CACHE_MINUTES:=15},
    "cache_size": ${STORE_CACHE_SIZE:=50},
    "allow_currency_conversion": ${ALLOW_CURRENCY_CONVERSION:=true},
    "currency_conversion_cache_size": ${CURRENCY_CACHE_SIZE:=10},
    "currency_conversion_cache_ttl_minutes": ${CURRENCY_CACHE_MINUTES:=120}
}
EOL

./booth2rss_web
