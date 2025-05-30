# Rust WebAssembly GitHub Page

This is a simple GitHub Pages site built with Rust and WebAssembly.

## Overview

This project demonstrates how to create a static website using Rust compiled to WebAssembly. The Rust code is compiled to WebAssembly using `wasm-pack`, and the resulting JavaScript and WebAssembly files are loaded by the HTML page.

## Project Structure

- `src/lib.rs`: The Rust code that will be compiled to WebAssembly
- `Cargo.toml`: The Rust package configuration
- `index.html`: The HTML page that loads the WebAssembly module
- `styles.css`: CSS styles for the page
- `.github/workflows/deploy.yml`: GitHub Actions workflow for building and deploying the site

## Local Development

To build and run this project locally, you'll need:

1. [Rust](https://www.rust-lang.org/tools/install)
2. [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

### Building

```bash
# Build the WebAssembly package
wasm-pack build --target web --out-name fox7fog_github_io --out-dir ./pkg
```

### Running Locally

After building, you can serve the site locally using any static file server. For example:

```bash
# Using Python's built-in HTTP server
python -m http.server
```

Then open your browser to http://localhost:8000

## Deployment

This project is automatically deployed to GitHub Pages when changes are pushed to the main branch. The GitHub Actions workflow in `.github/workflows/deploy.yml` handles the build and deployment process.

## License

This project is licensed under the MIT License - see the LICENSE file for details.