//swift-tools-version: 5.5
import PackageDescription

let package = Package(
    name: "MyPoint",
    products: [
        .library(name: "MyPoint", targets: ["MyPoint"]),
    ],
    targets: [
        .target(name: "MyPoint"),
    ]
)