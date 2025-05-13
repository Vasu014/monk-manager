# Initial Project Prompts for "monk-manager"

This document captures the initial set of prompts and specifications used to bootstrap the "monk-manager" AI coding assistant project.

## SPECS

We are going to create an AI coding assistant command line application in rust

The AI coding assistant is called "monk-manager".

It uses the "tracing" crate for logging, metrics and telemetry.
All operations have appropriate tracing on them that can be used to troubleshoot the application.

Use the clap cargo create for command line parsing.

The first operation is

"$ monkexplain"

When monkexplain is invoked it prints hello world.

IMPORTANT: Write up the specifications into the "specs/" folder with each domain topic (including technical topic) as a seperate markdown file. Create a "SPECS.md" in the root of the directory which is an overview document that contains a table that links to all the specs.

## CURSOR RULES

1.  Create a Cursor IDE AI MDC rule in ".cursor/rules" which instructs Cursor to always create new MDC rules in that folder. Each rule should be a seperate file.

2.  New Cursor IDE MDC rule.

    After each change performed by Cursor automatically from Git commit.

    Commit the changed files.

    Use the "conventional git commit convention" for the title of the commit message
    Explain what was changed and why the files were changed from exploring the prompts used to generate the commit.

3.  \[Language specific rules, Make it continue several times for a solid ruleset]
    Create a new Cursor MDC rule for all \*.rs files (in all subdirectories)

    You are an expert expert software engineer who knows rust. Infact you are the software engineer who created rust. Your task is to come up with technical recommendations in this rule which document best practices when authoring rust.

    Split each concern about rust into seperate MDC rules.

    Prefix each rule with the filename of "rust-$rulename.mdc"

    Write these rules to disk

4.  Look at the rust rules in @.cursor . What is missing? What does not follow best practice.

## IMPLEMENTATION

Study @SPECS.md for functional specifications.
Study @.cursor for technical requirements
Implement what is not implemented
Create tests
Run a "cargo build" and verify the application works 