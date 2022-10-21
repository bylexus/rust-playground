# http-server

This is my attempt to build a simple http server. It shall meet the following requirements SOMEDAY:

- multi-threaded: A Thread Pool of a fixed number of threads should answer the requests
- HTTP/1.0, 1.1: Should support HTTP 1.0 and 1.1 (keep-alive connections)
- proper http implementation (headers, parsing etc.)
- define routes:
  - for static files
  - with a callback function to handle the request individually
- request body as readable stream (now: an in-memory string)
- configuration:
  - routes
  - max post size
  - nr of threads
- Request and server logging (by supporting common formats, e.g. apache log format)

