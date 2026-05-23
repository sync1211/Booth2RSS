# Booth2RSS

A tool for creating RSS feeds from Booth.pm store pages. (And possibly more in the future!)

## Usage

### CLI
```bash
Booth2RSS.CLI <store-url> (--unblur-nsfw --allow-nsfw --include-unavailable --vrc-only)
```

### Web

Run `Booth2RSS.Web`, then access the RSS feed via:
https://localhost:8080/booth2rss/store?url=<store-url>

Optional arguments:
* `limit=<number>`: Maximum number of pages to fetch
* `allowNSFW=true|false`: Enable fetching of NSFW content.
* `unblurNSFW=true|false`: Disable the 18+ placeholder image on NSFW items to show the actual thumbnail.
* `vrcOnly=true|false`: Only show VRChat items
* `filter_unavailable=true|false`: Hide out-of-stock or discontinued items from the feed. (Defaults to `true`)
