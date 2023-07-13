# Building Block View

## Level 1 - Containers

buenzlimarks is mostly a standalone application.
Users interact with it through the web app built with React.
The web app installs itself with a service worker for offline use upon first visit.
Additionally, users may manually install a browser extension to interact with.
Both the web app and the browser extension access the browser's indexedDB for locally cached data.
Syncronization between different devices is achieved by talking to the Rust / Axum backend via HTTP and optionally websocket.
OAuth providers are used for lightweight user authentication.

The following container diagram shows a decomposition of the buenzlimarks software system into its main containers.
These containers are individually deployable and interact with each other only through protocols like HTTP & websocket.
For convenient deployment, the web app may be bundled with and served by the backend.

![C4 Container Diagram](../assets/gen/c4_container.png)

### Web app

The main GUI for interacting with buenzlimarks.
Installs itself via a service worker for offline use upon first visit.
Based on HTML, CSS, TypeScript, React, Tailwind.
Accesses user data locally via indexedDB, remotely via the REST API and may listen to remotely updated data via websocket.

### Backend server

Stores user data for synchronization across multiple devices.
It's written in Rust with the Axum framework and exposes a REST API.

### Browser extension

Offers a _subset_ of the features of the web app for convenient access anywhere in the browser.
Accesses user data locally via indexedDB, remotely via the REST API.

### Interfaces

The web app and browser extension have a somewhat implicit interface.
They need to agree on the structure of the data saved locally in indexedDB.
The backend server is mainly interacted with over its REST API which is _not_ explicitly documented, e.g. with openAPI.
In addition, the web app may open a websocket connection with the backend server, which will then notify the web app about changed data, e.g. by another device of the same user.

## Level 2 - Components

### Web app components

TODO document interal architecture of system components as it emerges.
Prefer relevance over completeness.
Specify important, surprising, risky, complex or volatile building blocks.

### Backend server components

The server is divided in three _layers_ or modules:
- `db`: The persistence layer or database, everything related to storing data to and fetching data from disk.
- `handlers`: A collection of functions, each responsible for "handling" a different kind of API request.
- `models`: The domain layer, including the data model and, if applicable, any associated business logic.

It deviates from a traditional layered architecture in that the handlers call the persistence layer directly.
The domain layer is only called in specific cases where business logic is actually involved.
This simplifies the majority of cases and avoids a lot of boilerplate, but may turn out to be more error prone if more business logic than expected is needed.

#### db (persistence layer)

Responsible for fetching and storing data in the database, be that a filesystem or a relational database.
Moreover, the persistence layer is responsible for ensuring relational validity of the data.
For example, if the database contains a bookmark referencing a widget with a given ID, a widget with such an ID must actually exist.
Lastly, the persistence layer may expose an API for simple data pre-processing including sorting and filtering.
This is good practice as relational databases are able to perform these operations very efficiently.

#### handlers

The handlers are bundled in hierarchical _routers_.
The routers determine which handler is responsible for a request with a given _route_ and _method_.
For example, the top-level router may contain one nested router for each domain entity (bookmark, widget, etc.) respectively.
These nested routers in turn may specify one handler for each of the four CRUD operations (create, read, update, delete).

### Browser Extension

TODO document interal architecture of system components as it emerges.
Prefer relevance over completeness.
Specify important, surprising, risky, complex or volatile building blocks.

## Level 3 - Code

Particularly complicated and or architecturally important code elements may be documented here.
