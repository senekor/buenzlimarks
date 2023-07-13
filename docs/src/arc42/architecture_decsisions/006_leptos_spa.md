# Leptos Single Page Application

## Context

We recently discovered that the relatively young Rust frontend framework Leptos is very mature, fully featured and well documented.
Since the reactivity model is very similar to React / SolidJs and Tailwind is supported, a migration doesn't entail too much work.
The advantage of such a migration is a complete Rust stack.
The downside is the loss of several libraries only found in the JavaScript world, notably tanstack-query and any headless UI libraries.

## Decision

We will switch our client-side UI framework from React to Leptos.

## Status

Accepted.

## Consequences

Finding high-quality libraries we need will be easier more difficult if not impossible.
Bundle size will be larger, due to compilation to WebAssembly.
The development environment and workflow will be much more streamlined.
There will be drastically fewer useless configuration files.
Our junior Rust-developer will be able to contribute to the frontend as well.

## Experience report
