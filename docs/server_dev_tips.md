# Server development workflow tips

This is a list of helpful tips and pointers to more resources related to developing the server.

## Getting started

First of all, run `just` in the terminal to get a list of the available recipes.
There should be one to run the server and one to fill your development database with seed data.

Note that the development database is in the directory `dev/db`.
You can use the file system directly to check the contents of the database when needed.

If you encounter any terms you don't understand, check the [glossary](./arc42/12_glossary.md).
If the term isn't in the glossary, ask about it so it can be added.

## Making sense of the server directory

There are two main perspectives that are needed to understand the server.
Both are documented well in the architecture documentation, find links to the specific sections below.

The first is the _static building block view_.
It is concerned with how the code is structured into modules (folders), the purpose of those modules and how they relate to each other.
The building block view is documented [here](./arc42/05_building_block_view.md#backend-server-components).

The second important view is the _dynamic runtime view_.
It is concerned with the sequence of events and lines of communication while the server is running.
The most important "unit of execution" for a web server is the API call.
A user makes an API call over the internet, meaning they ask the server to do something or provide some information.
They wait for the server to respond in some way.
The server handles such a request in a specific pattern.
To understand this pattern, refer to the [sequence diagram of an API call](./arc42/06_runtime_view.md#api-request) in the architecture documentation.

## I want to add a feature... how do I do that?

In order to add or change a feature, you will most likely have to change something in relation to every step outlined in the sequence diagram of an API call.

If you change everything at once and the result isn't what you want, you're gonna have a bad time looking for the problem.
Make sure you change only one thing at a time and verify that your change works at every step of the way.
The best way to do this is to write a unit test for your change.
The modules `handlers` and `db` already contain examples of unit tests.
Before your changes can be merged, they need to be unit tested anyway.
Doing that part first makes the rest of the work that much easier.

To test your changes with a proper API call, consider using the vscode extension [Thunder Client](https://github.com/rangav/thunder-client-support#usage).
An example API call that should work is already stored in the repository.
Make sure the server is running when you're making API calls.

Everything related to _routing_ is managed closely by the axum framework.
Without any knowledge of that, the routing can be hard to understand.
Refer to the [documentation of axum's `Router`](https://docs.rs/axum/latest/axum/struct.Router.html).

When adding or changing a handler function, you may sometimes get cryptic errors.
The router expects the handlers to conform to a generic type which can lead to bad error messages.
`axum` provides a macro that can improve these errors.
Simply add this line right above your handler function defintion: `#[axum::debug_handler]`.
