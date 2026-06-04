# Foody

A CLI meal planning and grocery management tool written in Rust.

## Features
Stated requirements can be found in (reqs.md)[reqs.md]

## Installation

```bash
cargo build --release
# or
cargo install --path .
```

## Setup

Initialize the SQLite database with `sqlx`:
```bash
cargo install sqlx-cli
sqlx database create
sqlx migrate run
```

## Quick Start

```bash
# Add a meal and assign ingredients
foody meal add "Pasta Bolognese"
foody ingredient assign "Pasta Bolognese" pasta,beef,tomato,garlic

# Create a plan and auto-fill for 7 days
foody plan add "Week 1"
foody plan fill "Week 1" --days 7

# Generate a shopping list and export it
foody grocery plan "Week 1" --export shopping_list.txt
```