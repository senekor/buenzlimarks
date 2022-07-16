# System Scope and Context

BuenzliMarks is mostly a standalone application. Users interact with it through
the web app built with Solid, which installs itself with a service worker for
offline use upon first visit. Additionally, users may manually install a browser
extension to interact with. Both the web app and the browser extension access
the browser's indexedDB for locally cached data. Syncronization between
different devices is achieved by talking to the Rust / Axum backend via REST and
or websocket. Some OAuth provider is used for lightweight user authentication.

<div>
    <img src="./images/context.svg" style="filter: invert(88%)"></img>
</div>
