// swift-tools-version:5.9
import PackageDescription

let package = Package(
    name: "AppleMetalSwiftBridge",
    platforms: [
        .macOS(.v11)
    ],
    products: [
        .library(
            name: "AppleMetalSwiftBridge",
            type: .static,
            targets: ["AppleMetalSwiftBridge"])
    ],
    targets: [
        .target(
            name: "AppleMetalSwiftBridge",
            path: "Sources/AppleMetalBridge")
    ]
)
