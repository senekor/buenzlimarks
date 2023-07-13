# Architecture Constraints

## Web technologies

The application must run within browsers, anything else would constitute bad UX.
The architecture is thus constrained to fulfill its goals with the web technologies available in modern browsers.
(e.g. HTML+CSS+JS+WASM, service worker capabilities, browser extension APIs)

## Limited server resources

The backend server must be self-hostable where compute resources may be limited like a raspberry pi.
It must therefore be relatively resource efficient, which constrains the choice of technology used for its development.
