Yoba web interface
==================

Yoba interpreter working on client side

To build it you need [wasm-pack](https://rustwasm.github.io/wasm-pack/book/).

Build with:

```sh
$ wasm-pack build --target no-modules
$ cp static/* pkg/
```

Then serve the `pkg/` dir with any web server.

Note: server must send `.wasm` extension with proper `application/wasm` mime-type.
