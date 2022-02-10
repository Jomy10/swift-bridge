// swift-tools-version:5.5

import PackageDescription

let package = Package(
    name: "SQLiteTest",
    dependencies: [
        .package(path: "../Csqlite3"),
    ],
    targets: [
        .executableTarget(
            name: "SQLiteTest",
            dependencies: ["Csqlite3"]),
    ]
)
