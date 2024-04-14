# fg-export

`fg-export` is a Rust-based Command Line Interface (CLI) tool designed for converting Figma designs into HTML and CSS code. It provides an efficient way for designers and developers to transform Figma components into web-ready formats, streamlining the development process and ensuring a smooth transition from design to code.

## Features
**Direct Figma Integration**: Connects to Figma using an access token to fetch design data.
**HTML & CSS Output**: Generates HTML files and CSS stylesheets based on Figma designs.
**Responsive Design Compatibility**: Exports are responsive, adapting to different screen sizes.
**Cache Option**: Allows for offline usage by leveraging cached Figma data.
**Rust-Based Performance**: Built with Rust for efficiency and speed.

## Installation
Before installing `fg-export`, ensure you have Rust installed on your system. You can install `fg-export` using `cargo`:

```bash
cargo install fg-export
```

## Usage
Basic usage of `fg-export` requires a Figma token and the Figma file key:

```bash
fg-export --token YOUR_FIGMA_TOKEN YOUR_FIGMA_FILE_KEY
```

### Options
* `-t, --token <TOKEN>`: Your Figma access token.
* `--cache`: Use cached data from figma_output/cache.json instead of fetching from the network.
* `-h, --help`: Display help information.
* `-V, --version`: Display the version of the tool.

### Examples
* Exporting from Figma API:

```bash
fg-export --token YOUR_FIGMA_TOKEN YOUR_FIGMA_FILE_KEY
```

* Exporting using cache:

```bash
fg-export --cache
```

## Contributing
We welcome contributions! Please refer to our contributing guidelines for detailed information on how you can contribute to `fg-export`.

## License
`fg-export` is licensed under the MIT License. See LICENSE for more information.

## Acknowledgments
Thanks to the Figma community and all the contributors who have made `fg-export` possible.

## Tokens - TODO
Create css variables based on figma tokens and design tokens (https://design-tokens.github.io/community-group/format/#introduction).
Figma variables can't be converted to css tokens without enterprise account to be able to use the REST API for Variables.

## TODO
- update CLI to generate styles with REM units instead of PX
- update CLI to pass a prefix for the components (ex: prefix=x for component button should create class: x-button), maybe this makes more sense for webcomponents
- generate webcomponents from styles and markup, markup generation will be different
- maybe update CLI to have a param to say that we want to generate webcomponents, or decide in a type of generation??
- generate lists markup for Text nodes
- GENERATE WEB COMPONENTS
- GENERATE TOKENS
    - Missing text and grids tokens and add them to component css
- GENERATE DESIGN_TOKENS
- GENERATE VARIABLES
- GENERATE SVG COMPONENTS
- GENERATE Colour gradient styles
- ADD unit tests 
- Test export with Uk gov figma designs https://www.figma.com/community/file/946837271092540314
- Publish CLI to Cargo
- Remove css duplicates
- Add missing children of a component as display none
- Add gradients
- Add Text list ???

