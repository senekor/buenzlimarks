# Building Block View

## Level 1 - Containers

buenzlimarks is mostly a standalone application.
Users interact with it through the web app built with Solid
The web app installs itself with a service worker for offline use upon first visit.
Additionally, users may manually install a browser extension to interact with.
Both the web app and the browser extension access the browser's indexedDB for locally cached data.
Syncronization between different devices is achieved by talking to the Rust / Axum backend via HTTP and optionally websocket.
OAuth providers are used for lightweight user authentication.

The following container diagram shows a decomposition of the buenzlimarks software system into its main containers.
These containers are individually deployable and interact with each other only through protocols like HTTP & websocket.
For convenient deployment, the web app may be bundled with and served by the backend.

<div style="border: solid; display: flex; flex-direction: column">
  <h2 style="align-self: center">C4 Container Diagram</h2>
  <img src="./diagrams/container.svg" />
</div>

### Web app

The main GUI for interacting with buenzlimarks.
Installs itself via a service worker for offline use upon first visit.
Based on HTML, CSS, TypeScript, Solid, Tailwind.
Accesses user data locally via indexedDB, remotely via the REST API and may listen to remotely updated data via websocket.

### Backend server

Stores user data for synchronization across multiple devices.
Written in Rust with the Axum framework.
Exposes a REST API.

### Browser extension

Offers a _subset_ of the features of the web app for convenient access anywhere in the browser.
Accesses user data locally via indexedDB, remotely via the REST API.

### Interfaces

The web app and browser extension have a somewhat implicit interface.
They need to agree on the structure of the data saved locally in indexedDB.
The backend server is mainly interacted with over its REST API which is _not_ explicitly documented, e.g. with openAPI.
In addition, the web app may open a websocket connection with the backend server, which will then notify the web app about changed data, e.g. by another device of the same user.

## Level 2 - Components

### Web App

TODO document interal architecture of system components as it emerges.
Prefer relevance over completeness.
Specify important, surprising, risky, complex or volatile building blocks.

### Backend server

TODO document interal architecture of system components as it emerges.
Prefer relevance over completeness.
Specify important, surprising, risky, complex or volatile building blocks.

### Browser Extension

TODO document interal architecture of system components as it emerges.
Prefer relevance over completeness.
Specify important, surprising, risky, complex or volatile building blocks.

## Level 3 - Code

Particularly complicated and or architecturally important code elements may be documented here.
