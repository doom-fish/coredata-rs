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
            name: "CoreDataObjCBridge",
            path: "Sources/CoreDataObjCBridge",
            publicHeadersPath: "include"),
        .target(
            name: "CoreDataBridge",
            dependencies: ["CoreDataObjCBridge"],
            path: "Sources/CoreDataBridge",
            publicHeadersPath: "include")
    ]
)
