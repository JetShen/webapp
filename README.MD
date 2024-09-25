# Rust Web Application

This project is a simple web application written in Rust. The purpose of this project is to study the Rust programming language and understand how to build web applications using Rust.

## Overview

The application uses the `Nickel` web framework to create a simple web server that logs the current date and time to a specified log file whenever it receives a GET request. The log entries are formatted using the `chrono` crate, and command-line arguments are parsed using the `clap` crate.

## Dependencies

The project relies on the following external crates:
- `nickel`: A lightweight web framework for Rust.
- `chrono`: A date and time library for Rust.
- `clap`: A library for parsing command-line arguments.
