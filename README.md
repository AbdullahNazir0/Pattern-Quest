# Pattern Quest

Pattern Quest is a simple Rust-based grep clone that allows you to search for patterns in text files with additional features like case-insensitive searching and line number display.

## Table of Contents

- [Introduction](#introduction)
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Configuration](#configuration)
- [Planned Features](#planned-features)
- [Contributing](#contributing)
- [License](#license)

## Introduction

Pattern Quest is a lightweight command-line utility inspired by grep, designed to search for patterns in text files. It provides additional features like case-insensitive searching and the ability to display line numbers.

## Features

1. **Case-Insensitive Searching:**
   - Enable case-insensitive searching by setting the `IGNORE_CASE` environment variable.

2. **Show Line Numbers:**
   - Display line numbers alongside search results.

3. **Invert Search Results:**
   - Invert the search results by setting the `INVERT_SEARCH` environment variable.

## Installation

To use Pattern Quest, follow these installation steps:

```bash
# Clone the repository
git clone https://github.com/yourusername/pattern-quest.git

# Navigate to the project directory
cd pattern-quest

# Build the project
cargo build

# Run the executable
cargo run -- --query "example" --file "path/to/example.txt"
```

## Usage

Pattern Quest supports the following command-line options:

```bash
# Example usage with case-insensitive searching and inverted searching
IGNORE_CASE=1 INVERT_SEARCH=1 cargo run -- --query "example" --file "path/to/example.txt"
```

## Configuration

Pattern Quest can be configured using environment variables. Set these variables based on your preferences:

### Environment Variables

- **INVERT_SEARCH**
  - Set this environment variable to enable inverted searching.
 
  ```bash
  export INVERT_SEARCH=true
  ```

- **IGNORE_CASE:**
  - Set this environment variable to enable case-insensitive searching.

  ```bash
  export IGNORE_CASE=true
  ```

## Planned Features

The following features are planned to be added in future releases:

### Redirect Output to a File:

Allow users to redirect search results to a specified file.

### Help/Usage Information:

Provide help and usage information using the --help option.

Stay tuned for updates as we work on enhancing Pattern Quest!

## Contributing
Contributions to Pattern Quest are highly encouraged and appreciated. Here's how you can contribute:

### Issues:

Submit issues for bugs, feature requests, or enhancements on the Issue Tracker.

### Pull Requests:

Contribute directly by opening pull requests. Ensure your changes align with the CONTRIBUTING.md guidelines.
Guidelines:

Follow the guidelines outlined in the CONTRIBUTING.md file for coding standards, testing, and documentation.
By contributing, you play a vital role in improving Pattern Quest for the entire community!

## License
Pattern Quest is licensed under the MIT License. See the LICENSE file for details.
