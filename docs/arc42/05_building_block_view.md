# Building Block View

## Level 1 - Containers

BuenzliMarks is mostly a standalone application. Users interact with it through
the web app built with Solid, which installs itself with a service worker for
offline use upon first visit. Additionally, users may manually install a browser
extension to interact with. Both the web app and the browser extension access
the browser's indexedDB for locally cached data. Syncronization between
different devices is achieved by talking to the Rust / Axum backend via REST and
or websocket. Some OAuth provider is used for lightweight user authentication.

The following container diagram shows a decomposition of the BuenzliMarks
software system into its main containers. These containers are individually
deployable and interact with each other only through protocols like HTTP &
websocket.

<div>
    <img src="./diagrams/container.svg"></img>
</div>

### Web App

The main GUI for interacting with Buenzlimarks. Installs itself via a service
worker for offline use upon first visit. Based on HTML, CSS, TypeScript, Solid,
Tailwind. Accesses user data locally via indexedDB, remotely via the REST api
and may listen to remotely updated data via websocket.

### Sync Service

Stores user data for synchronization across multiple devices. Based on Rust,
Axum. Exposes a REST api.

### Browser Extension

Offers a _subset_ of the features of the web app for convenient access anywhere
in the browser, i.e. creating new bookmarks. Accesses user data locally via
indexedDB, remotely via the REST api.

### Interfaces

The web app and browser extension have a somewhat implicit interface insofar as
the need to agree on the structure of the data saved locally in indexedDB. The
sync service is mainly interacted with over its REST api which is _NOT_ explicitly
documented, e.g. with openAPI. In addition, the web app may open a websocket
connection with the sync service which will then notify the web app about changed
data, e.g. by another device of the same user.

## Level 2 - Components

TODO document interal architecture of system components as it emerges.

Here you can specify the inner structure of (some) building blocks from level 1
as white boxes.

You have to decide which building blocks of your system are important enough to
justify such a detailed description. Please prefer relevance over completeness.
Specify important, surprising, risky, complex or volatile building blocks. Leave
out normal, simple, boring or standardized parts of your system

### Web App

### Sync Service

### Browser Extension

## Level 3 - Code

Particularly complicated and or architecturally important code elements may
be documented here.
