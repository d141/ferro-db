# FerroDB

FerroDB is a custom-built, lightweight key-value store designed to provide an efficient and straightforward method for storing and retrieving data via both an interactive shell and a web API.

## Goal of the Project

The primary goal of FerroDB is to offer a simple yet functional key-value database system with an emphasis on ease of use and accessibility.

## Current State

As of the current release, FerroDB supports basic CRUD (Create, Read, Update, Delete) operations within a persisted key-value store. The database ensures data persistence through file-based storage and provides an optional mechanism for handling transactions and changes.

### Features

- **CRUD Operations:** Set, get, and unset values within collections.
- **Data Persistence:** Save and load the database state to and from disk.
- **Transaction Logging:** Basic support for tracking changes and rollback capabilities.
- **Interactive Shell:** Execute database commands through a command-line interface.
- **Web API:** Access the database programmatically via HTTP endpoints.

### Roadmap

- Full ACID compliance
- Rewinding and Time Traveling
- Full Query Language

## Installation

To install and run FerroDB, follow these steps:

1. Clone the repository:

   ```
   git clone https://github.com/d141/ferrodb.git
   cd ferrodb
   ```

    1a. Build the project using Cargo (ensure Rust is installed):

        cargo build --release

    1b. Run the server:

        cargo run -- server

    1c. Alternatively, interact with the database using the shell:

        cargo run -- shell

## Usage

### Via Interactive Shell
Start the shell:

`make shell`

Example commands:

    - CREATE mycollection - Creates a new collection named 'mycollection'.
    - SET mycollection mykey myvalue - Sets 'mykey' to 'myvalue' in 'mycollection'.
    - GET mycollection mykey - Retrieves the value associated with 'mykey'.

### Via API

Ensure the server is running:

`make server`

Example curl commands:

Set a value:

`curl -X POST -d "value" "http://localhost:31415/set/mycollection/mykey"`

Get a value:

`curl "http://localhost:31415/get/mycollection/mykey"`

Unset a value:

`curl -X DELETE "http://localhost:31415/unset/mycollection/mykey"`

### License
FerroDB is released under the MIT License. See the LICENSE file for more details.
