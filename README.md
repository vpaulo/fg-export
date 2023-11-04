Figma exporter CLI
=======================

The Figma exporter CLI is a tool to generate CSS and HTML from Figma designs.

# Usage

```
Usage: fg-xport [OPTIONS] --token <TOKEN> <FILE>

Arguments:
  <FILE>  Figma file

Options:
  -t, --token <TOKEN>  Figma access token
      --cache          If set, don't connect to the network, but use the `figma_output/cache.json`
  -h, --help           Print help
  -V, --version        Print version

Examples:
  Figma REST API call
    $ fg-xport --token 5dfd6626-ab1d-42da-bb76-90def3153998 5dfd6626ab1d

  From cache
    $ fg-xport --token 5dfd6626-ab1d-42da-bb76-90def3153998 --cache 5dfd6626ab1d
```
