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
    $ fg-xport --token <TOKEN> <FILE>

  From cache
    $ fg-xport --token <TOKEN> --cache <FILE>
```

## Tokens - TODO
Create css variables based on figma tokens and design tokens (https://design-tokens.github.io/community-group/format/#introduction).
Figma variables can't be converted to css tokens without enterprise account to be able to use the REST API for Variables.

## TODO
- format css files
- create html files
- update CLI to run from cache without passing token and file in the command
- update CLI to generate styles with REM units insted of PX
- add auto layout styles
- create children styles, childrean need to know the parent because of auto layout and other dependencies in styles
- update CLI to pass a prefix for the components (ex: prefix=x for component button should create class: x-button), maybe this makes more sense for webcomponents
- generate webcomponents from styles and markup, markup generation will be different
- maybe update CLI to have a param to say that we want to generate webcomponents, or decide in a type of generation??
