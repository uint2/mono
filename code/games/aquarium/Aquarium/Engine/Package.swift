// swift-tools-version: 5.8
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "Engine",
    products: [
        .library(name: "Engine", targets: ["Engine"]),
        .executable(name: "EngineCli", targets: ["EngineCli"]),
        .executable(name: "Csp", targets: ["Csp"])
    ],
    dependencies: [],
    targets: [
        .target(
            name: "Engine",
            dependencies: []),
        .testTarget(
            name: "EngineTests",
            dependencies: ["Engine"],
            path: "Tests"),
        .executableTarget(
            name: "EngineCli",
            dependencies: ["Engine"]),
        .executableTarget(
            name: "Csp",
            dependencies: [])
    ])
