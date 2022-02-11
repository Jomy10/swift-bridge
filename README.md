## Set up rust project
```yaml
# Cargo.toml
[lib]
crate-type = ["staticlib"]

[build-dependencies]
swift-bridge-build = "0.1"

[dependencies]
swift-bridge = "0.1"
```

```rust
// src/lib.rs

#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        fn print_hello_rust();
    } 
}

fn print_hello_rust() {
    println!("Hello from Rust!");
}
```

```rust
// build.rs

use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from("generated");
    let bridges = vec!["src/lib.rs"];
    for path in &bridges {
        println!("cargo:rerun-if-changed={}", path);
    }
    
    swift_bridge_build::parse_bridges(bridges)
        .write_all_concatenated(out_dir, env!("CARGO_PKG_NAME"));
}
```

Add build script

```bash
# build.sh

#!/bin/bash
set -e

export SWIFT_BRIDGE_OUT_DIR="$(pwd)/generated"

cargo build --target x86_64-apple-darwin
```

build the header and swift files
```bash
./build.sh
```

## Create the xcfamework

Make a new directory (`framework`).

Copy the the `.a` file and the headers to this directory.

The folder structure should look like this:

```
framework
├── include
│   ├── SwiftBridgeCore.h
│   └── rust.h
└── librust.a
```

### Add a module map in the include directory

*include/module.modulemap*
```
module Rust {
    header "rust.h"
    header "SwiftBridgeCore.h"
    export *
}
```

Inside of the new folder, run:
```bash
xcodebuild -create-xcframework -output rust.xcframework  \
    -library librust.a \
    -headers include
```

## Swift Package
Let's now make another folder for our package and run `swift package init --type library` inside of it.

Let's copy the generated swift files to the `Sources/{package_name}` directory and add `import Rust` to them and make their functions public.

Copy the xcframework to our new folder.

Edit the Package.swift:

```swift
// swift-tools-version:5.5

import PackageDescription

let package = Package(
    name: "package",
    products: [
        .library(
            name: "package",
            targets: ["package"]),
    ],
    dependencies: [

    ],
    targets: [
        .binaryTarget(
            name: "Rust",
            path: "rust.xcframework"
        ),
        .target(
            name: "package",
            dependencies: ["Rust"]),
    ]
)
```

Folder structure:
```
package
├── Package.swift
├── Sources
│   └── package
│       ├── SwiftBridgeCore.swift
│       └── rust.swift
└── rust.xcframework
    ├── Info.plist
    └── macos-x86_64
        ├── Headers
        │   ├── SwiftBridgeCore.h
        │   ├── module.modulemap
        │   └── rust.h
        └── librust.a
```

## Test Swift Package
Create a new folder and run `swift package init --type executablee`.

Edit the Package.swift

```swift
// swift-tools-version:5.5

import PackageDescription

let package = Package(
    name: "testPackage",
    products: [
        .library(
            name: "testPackage",
            targets: ["testPackage"]),
    ], 
    dependencies: [
        .package(path: "../package")
    ],
    targets: [
        .target(
            name: "testPackage",
            dependencies: [
                .product(name: "package", package: "package")
            ])
    ]
)
```

*main.swift*
```swift
import package

print_hello_rust()
```

```
$ swift run
Hello from Rust!
```
