# Architecture Constraints

## Web technologies

The application must run within browsers, anything else would constitute bad UX. The architecture is thus constrained to fulfill its goals with the web technologies available in modern browsers. (e.g. HTML+CSS+JS, service worker capabilities, browser extension APIs)

## Few server resources

The synchronization service must be self-hostable where compute resources may be limited, e.g. on a raspberry pi. It therefore cannot afford to be unnecessarily resource intesive, which limits the choice of technology used for its development.
