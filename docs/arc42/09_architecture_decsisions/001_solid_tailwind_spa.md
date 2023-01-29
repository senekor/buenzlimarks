# SolidJS & TailwindCSS Single Page Application

## Context

Writing a web app with JavaScript basically requires the use of a UI framework, as vanilla JavaScript is incredibly unproductive and leads to unmaintainable UI code.
The lead developer has experience with React, which enables faster development.
[SolidJS](https://www.solidjs.com/) is similar to React from a development perspective, transitioning to it should have a gentle learning curve.
Solid is much newer more "cutting-edge", but also less mature in many ways.
In comparison to React, Solid:
- has a better developer experience
- requires less boilerplate
- sports better performance
- is particularly lacking in component libraries, which can be mitigated by choosing a framework-agnostic styling system like [TailwindCSS](https://tailwindcss.com/).

Framework popularity may be important for projects looking to hire new people, which is not the case for buenzlimarks.
The lead developer has neither experience with, nor interest in, other frameworks than React and Solid.

Tailwind generally requires more work for, but gives more control over, the styling of the web app compared to more "batteries-included" component libraries like Material-UI.
Component libraries on top of Tailwind do exist, one example being [daisyUI](https://daisyui.com/).

## Decision

We will write the web app with Solid and Tailwind.

## Status

Accepted.

## Consequences

Writing the web app will be a learning experience.
However, it will also generally be more work and take longer.
The resulting code is expected to be more maintainable.
Styling customization as a user-facing feature can be provided easily.

## Experience report
