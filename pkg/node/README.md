# usaddress-rs-wasm

A WebAssembly wrapper for the [us-addrs](https://github.com/raphaellaude/us-addrs/) Rust package, providing fast and accurate parsing of unstructured United States address strings into address components.

## Overview

usaddress-rs-wasm brings the power of the us-addrs Rust crate to the web, allowing you to parse US addresses directly in the browser or in Node.js applications. This project wraps the core functionality of us-addrs in a WebAssembly module, making it easy to integrate into JavaScript and TypeScript projects.

## Features

- Parse unstructured US address strings into structured components
- High-performance address parsing using Rust and WebAssembly
- Easy integration with web and Node.js applications
- Compatible with both synchronous and asynchronous usage patterns

## Installation

```bash
npm install usaddress-rs-wasm
```

## Usage

```javascript
const { parse } = require('usaddress-rs-wasm');

const address = "123 Main St, Anytown, CA 12345";
const result = parse(address);

console.log(result);
```

## API

### `parse(address: string): ParseResult`

Parses the given address string and returns a `ParseResult` object.

```typescript
type ParseResult = 
  | { data: Array<[string, string]> }
  | { error: string };
```

- If parsing is successful, the `data` property will contain an array of tuples, where each tuple represents a parsed component of the address. The first element of the tuple is the component value, and the second element is the component type.
- If an error occurs during parsing, the `error` property will contain an error message.

## Example

```javascript
const { parse } = require('usaddress-rs-wasm');

const address = "33 Nassau Avenue, Brooklyn, NY";
const result = parse(address);

if ('data' in result) {
  console.log("Parsed address components:");
  for (const [value, type] of result.data) {
    console.log(`${type}: ${value}`);
  }
} else {
  console.error("Error parsing address:", result.error);
}
```

Output:
```
Parsed address components:
AddressNumber: 33
StreetName: Nassau
StreetNamePostType: Avenue
PlaceName: Brooklyn
StateName: NY
```

## Building from Source

To build the WebAssembly module from source, you'll need to have Rust and wasm-pack installed. Then, follow these steps:

1. Clone the repository:
   ```
   git clone https://github.com/username/usaddress-rs-wasm.git
   cd usaddress-rs-wasm
   ```

2. Build the WebAssembly module:
   ```
   wasm-pack build --target web
   ```

3. The built files will be in the `pkg` directory, ready for use or distribution.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- This project is based on the [us-addrs](https://github.com/raphaellaude/us-addrs/) Rust crate, which is itself inspired by the [usaddress](https://github.com/datamade/usaddress) Python library.
- Thanks to the [datamade](https://datamade.us/) team for their work on the original usaddress library.
