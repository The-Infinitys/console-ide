# Extension System for Console IDE

This document outlines the design and functionality of the extension system within Console IDE.

## Overview

The Console IDE supports a robust extension system that allows users to extend its functionality through external binaries. These extensions are discovered, resolved for dependencies, and executed as background processes, communicating with the host IDE via `instance-pipe`.

## Extension Discovery and Loading

1.  **Extension Directory**: Console IDE scans the `~/.console-ide/extensions` directory for executable binaries. Each binary found in this directory is considered a potential extension.
2.  **Metadata Retrieval**: For each discovered binary, Console IDE executes the command `{binary} console-ide info relation`. This command is expected to output a JSON string containing the extension's metadata, including its ID, version, dependencies, and conflicts. The metadata is defined by the `Extension` struct in `console-ide-feature`.

## Dependency Resolution

Console IDE utilizes a sophisticated dependency resolution mechanism to ensure that all active extensions are compatible and their requirements are met. This process involves:

1.  **Dependency Graph Construction**: All discovered extensions and their declared dependencies are used to build a dependency graph.
2.  **Resolution Logic**: The `Resolver` component (from `console-ide-feature`) performs the following checks:
    *   **Missing Dependencies**: Verifies that all declared dependencies for an extension are available.
    *   **Version Compatibility**: Ensures that the versions of dependent extensions satisfy the specified version ranges (e.g., `^1.0.0`, `>=0.5.0`).
    *   **Circular Dependencies**: Detects and prevents scenarios where extensions form a circular dependency chain.
    *   **Conflicts**: Identifies and flags extensions that declare conflicts with each other.
3.  **Error Reporting**: If any resolution errors are encountered, they are reported to the user, preventing the activation of incompatible extensions.

## Extension Activation and Communication

1.  **Background Processes**: Successfully resolved extensions are activated by executing them as background processes using the command `{binary} console-ide exec background-process`.
2.  **`instance-pipe` Communication**: Each activated extension establishes a communication channel with the host IDE using the `instance-pipe` library. A unique pipe name is generated for each extension (e.g., `console-ide-pipe-{extension_id}`).
3.  **Message Protocol**:
    *   **Host to Extension (`HostMessage`)**: The host can send commands to extensions, such as `SpawnWidget` to request the creation of a UI widget.
    *   **Extension to Host (`ExtensionMessage`)**: Extensions can send events or responses back to the host, such as `WidgetSpawned` to confirm the creation of a widget.
4.  **Widget Spawning**: Extensions can request the host to spawn UI widgets. The host then handles the rendering and management of these widgets within its TUI (Text User Interface).

## Extension Development

To create a Console IDE extension:

1.  Develop a Rust binary (or any executable that can respond to the CLI commands).
2.  Implement the `info relation` subcommand to output `Extension` metadata as JSON.
3.  Implement the `exec background-process` subcommand to contain the extension's main logic and use `instance-pipe` for communication with the host.
4.  Place the compiled binary in the `~/.console-ide/extensions` directory.

## Future Enhancements

*   More sophisticated message protocols for richer host-extension interactions.
*   Dynamic loading and unloading of extensions.
*   Extension sandboxing for enhanced security.
*   User interface for managing extensions (install, enable, disable).
