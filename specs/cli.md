# CLI Interface Specification

## Overview
This document outlines the command-line interface (CLI) design for `monk-manager`. It covers the general structure, global options, and initial setup flow.

## Initial Setup: Setting the Repository Home

Upon the first invocation in a new terminal session or if a repository home has not been previously set for the current workspace context, `monk-manager` will prompt the user to specify the path to their project's root directory.

```
monk-manager
> No repository home set for this workspace.
> Please enter the absolute path to your project's root directory: /path/to/your/project
> Repository home set to: /path/to/your/project
```

*   This path will be stored (e.g., in a local configuration file or session state) and used as the default base for all file operations and context gathering in subsequent commands within that workspace/session.
*   Users should be able to change or override this path, perhaps with a global command like `monk-manager set-repo-home /new/path/to/project` or an environment variable.

This ensures that `monk-manager` always has a well-defined root for resolving relative paths and understanding project structure.

## General Command Structure

All `monk-manager` commands will follow a consistent structure:

```bash
monk-manager [GLOBAL_OPTIONS] <COMMAND> [COMMAND_OPTIONS] [ARGUMENTS]
```

*   `[GLOBAL_OPTIONS]`: Options that apply to `monk-manager` as a whole (e.g., `--verbose`, `--config-file`).
*   `<COMMAND>`: The specific action to be performed (e.g., `explain`, `edit`, `init`).
*   `[COMMAND_OPTIONS]`: Options specific to the `<COMMAND>`.
*   `[ARGUMENTS]`: Arguments required by the `<COMMAND>`.

## Available Commands

*   `explain`: (See `explain-command.md`) Provides explanations for code snippets.
*   `edit`: (See `edit-command.md`) Allows AI-driven code modifications.
*   `(More commands to be added here as they are defined)`

## Global Options

*   `--help`, `-h`: Displays help information for `monk-manager` or a specific command.
*   `--version`, `-V`: Displays the current version of `monk-manager`.
*   `--verbose`, `-v`: Enables verbose output for more detailed logging and diagnostics. (The level of verbosity could be controlled, e.g. -vv, -vvv).
*   `--config <CONFIG_FILE_PATH>`: Specifies a custom path to a configuration file.

## Output and Formatting

*   CLI output should be clear, concise, and well-formatted.
*   Use standard streams appropriately (stdout for results, stderr for errors and logs).
*   Consider using colors or styling to improve readability where appropriate (e.g., for diffs, errors, success messages), with an option to disable styling (e.g., `NO_COLOR` environment variable). 