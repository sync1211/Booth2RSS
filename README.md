# Booth2RSS

A tool for creating RSS feeds from Booth.pm store pages.

## Usage

### CLI
```bash
Booth2RSS.CLI <store-url> (--unblur-nsfw --allow-nsfw --include_unavailable --vrc-only --limit=<max-items> --max-pages=<max-pages>)
```

### Web

Run `Booth2RSS.Web`, then access the RSS feed via:
https://localhost:8080/booth2rss/store?url=<store-url>

Optional arguments:
* `limit=<number>`: Maximum number of pages to fetch
* `allow_nsfw=true|false`: Enable fetching of NSFW content.
* `unblur_nsfw=true|false`: Disable the 18+ placeholder image on NSFW items to show the actual thumbnail.
* `vrc_only=true|false`: Only show VRChat items
* `filter_unavailable=true|false`: Hide out-of-stock or discontinued items from the feed. (Defaults to `true`)
