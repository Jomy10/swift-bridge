// swift-tools-version:5.5
import PackageDescription

let package = Package(
    name: "Sample",
    products: [
        .executable(name: "Sample", targets: ["Sample"]),
    ],
    dependencies: [
        .package(path: "../MyPoint")
    ],
    targets: [
        .target(name: "Sample", dependencies: [
            .product(name: "MyPoint", package: "MyPoint")
        ])
    ]
)