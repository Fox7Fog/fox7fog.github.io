# Pure Rust Web Development

This is a GitHub Pages site built with 100% Rust, using Yew and WebAssembly.

## Overview

This project demonstrates how to create a modern web application using pure Rust. The entire UI, including HTML structure and CSS styling, is written in Rust code using the Yew framework and compiled to WebAssembly. This approach eliminates the need for separate HTML and CSS files, allowing for a fully type-safe, Rust-driven development experience.

## Project Structure

- `src/lib.rs`: The Rust library code that defines the Yew components and styles
- `src/main.rs`: The entry point for the Yew application
- `Cargo.toml`: The Rust package configuration
- `index.html`: A minimal HTML shell that loads the WebAssembly application
- `Trunk.toml`: Configuration for the Trunk build tool
- `.github/workflows/deploy.yml`: GitHub Actions workflow for building and deploying the site

## Features

- **100% Rust**: The entire application is written in Rust
- **Zero JavaScript**: No JavaScript code is needed
- **Type Safety**: Leverage Rust's powerful type system for your web UI
- **Dark/Light Theme**: Built-in theme switching functionality
- **Responsive Design**: Works on mobile and desktop devices

## Local Development

To build and run this project locally, you'll need:

1. [Rust](https://www.rust-lang.org/tools/install)
2. [Trunk](https://trunkrs.dev/) - Install with `cargo install trunk`
3. WebAssembly target - Install with `rustup target add wasm32-unknown-unknown`

### Building and Running Locally

```bash
# Start the development server
trunk serve

# Or build for production
trunk build --release
```

The development server will automatically open your browser to http://localhost:8080

## Deployment

This project is automatically deployed to GitHub Pages when changes are pushed to the main branch. The GitHub Actions workflow in `.github/workflows/deploy.yml` handles the build and deployment process.

## Why Pure Rust?

Using Rust for web development offers several advantages:

1. **Safety**: Rust's ownership model prevents common bugs like null pointer dereferences
2. **Performance**: WebAssembly provides near-native performance
3. **Consistency**: Use the same language for both frontend and backend
4. **Developer Experience**: Enjoy Rust's excellent tooling and compiler feedback

## License

This project is licensed under the MIT License - see the LICENSE file for details.
