// swift-tools-version:5.9
import PackageDescription

let package = Package(
    name: "CoreDataBridge",
    platforms: [
        .macOS(.v13)
    ],
    products: [
        .library(
            name: "CoreDataBridge",
            type: .static,
            targets: ["CoreDataBridge"])
    ],
    targets: [
        .target(
            name: "CoreDataBridge",
            path: "Sources/CoreDataBridge",
            publicHeadersPath: "include")
    ]
)
