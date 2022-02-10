# Packaging a system library into a Swift package

In `Csqlite3`, we link to the system library with a header file and a modulemap. In the `Pacakge.swift`, we then specify the systemLibrary target:

```swift
// ...
    targets: [
        .systemLibrary(name: "Csqlite3"),
    ]
// ...
```

We then use the Swift pacakge in `SQLiteTest`:

*Package.swift*
```swift
let package = Package(
    name: "SQLiteTest",
    // Declare dependency
    dependencies: [
        .package(path: "../Csqlite3"),
    ],
    targets: [
        .executableTarget(
            name: "SQLiteTest",
            // Add dependency to our executable target (which executes `Sources/SQLiteTest/main.swift`)
            dependencies: ["Csqlite3"]),
    ]
)
```

We can then use our system library in our `main.swift`:

```swift
// The name specified by the modulemap
import CSQLite

print("Hello, world!")
let version = String(cString: sqlite3_libversion())
print("SQLite3 Version: \(version)")
```

```bash
SQLiteTest$ swift run
Hello, world!
SQLite3 Version: 3.36.0
```
