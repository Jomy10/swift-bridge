// swift-tools-version:5.5

import PackageDescription

let package = Package(
    name: "testPackage",
    dependencies: [
        .package(path: "../package")
    ],
    targets: [
        .executableTarget(
            name: "testPackage",
            dependencies: [
                .product(name: "package", package: "package")
            ])
    ]
)
