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
