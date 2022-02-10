# Using a C library in Swift

In `MyPoint`, we have a C source file that includes `MyPoint.h`

*MyPoint.h*
```header
struct MyPoint {
    int x;
    int y;
};
```

We declare the target in our *Package.swift*:
```swift
let package = Package(
    name: "MyPoint",
    products: [
        .library(name: "MyPoint", targets: ["MyPoint"]),
    ],
    targets: [
        .target(name: "MyPoint"),
    ]
)
```

We then use the library in `Sample`:

*Package.swift*:
```swift
// ...
    dependencies: [
        .package(path: "../MyPoint")
    ],
    targets: [
        .target(name: "Sample", dependencies: [
            .product(name: "MyPoint", package: "MyPoint")
        ])
    ]
// ...
```

```bash
sample$ swift run
Hello, World 10 20
```
