## TODOs

- Incorporate service name so it shows up nicely in Jaeger
- Integ test that starts a jaeger container, uploads spans, and queries API asserting number of spans

## Notes

- Where's the right spot to fallibly parse a json serialized `SpanData`, but still operate over an Iterator?
- Do we need batch exporter?
- Refactor out refs to `SpanData` to one central type/alias in a "models" crate?

