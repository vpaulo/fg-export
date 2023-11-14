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
fg-export --token YOUR_FIGMA_TOKEN --cache YOUR_FIGMA_FILE_KEY
```

## Contributing
We welcome contributions! Please refer to our contributing guidelines for detailed information on how you can contribute to `fg-export`.

## License
`fg-export` is licensed under the MIT License. See LICENSE for more information.

## Acknowledgments
Thanks to the Figma community and all the contributors who have made `fg-export` possible.