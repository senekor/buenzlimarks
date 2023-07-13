# React Single Page Application

## Context

In order to speed up development, component libraries are often used.
These are not useable in our context, because we need full control over styling and have chosen Tailwind for this reason.
A more recent alternative to component libraries are headless UI libraries, which provide the logic and accessibility of UI components without any styling.
Radix is one of the leading headless UI libraries and it only supports React.

The supposedly better developer experience of SolidJS has been less advantageous than anticipated, as described in [this experience report](./001_solid_tailwind_spa.md#experience-report).

Solid and React remain very similar frameworks in terms of the code we write, migrating back and forth requires relatively little effort.

## Decision

We will switch our client-side UI framework from SolidJS to React.

## Status

Superceded by [React Single Page Application](./005_react_spa.md).

## Consequences

Finding high-quality libraries we need will be easier (e.g. Radix UI).
Performance will be worse, though likely unnoticeable.

## Experience report

SolidJs or React, JavaScript is just annoying.
