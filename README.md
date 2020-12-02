# web-bloom

A search engine embedded in your web page with the speed of a local search!

`web-bloom` is a proof of concept for a lightweight alternative to traditional heavy search engine.
Here your request is handled locally within the web browser without going through a third party service. No calls through the Internet, no network latency and no plugin.
The power of a local search engine installed directly on the computer of all your visitors!

`web-bloom` is designed to be fast and memory efficient at the expense of a customizable probability of false positives.
It relies on [index-bloom](https://github.com/odespesse/index-bloom) to implement a probabilistic search engine and then it is transposed into WebAssembly for execution in a web browser.
An index dump can be created easily with [cli-bloom](https://github.com/odespesse/cli-bloom) and imported as a regular Json file in the browser.

## Quick start

1. Transform a dump file into importable Json object:
```bash
$ echo -e "export default $(cat dump.json)" > index_dump.js
```
2. Copy your dump to the `public` directory:
```bash
$ mv index_dump.js public/
```
3. Compile the project with `wasm-pack`:
```bash
$ wasm-pack build --target web --out-dir public/pkg
```
4. Use a local web server (mandatory to bypass CORS restrictions):
```bash
$ docker run --rm --name web-bloom-nginx -v ${PWD}/public:/usr/share/nginx/html:ro -d -p 8080:80 nginx:1.19-alpine
```
5. Go to `http://localhost:8080`
6. The sample index at `public/index_dump.js` ingested books of William Shakespeare from the [Gutenberg project](https://www.gutenberg.org). You can search words present in:
- Hamlet
- Julius Caesar
- King Lear
- Macbeth
- Othello
- Rome and Juliet
The original source represent 968923 bytes and the optimized index is only 263643 bytes, a ~73% win!

## License

`web-bloom` is released under the MIT license ([LICENSE](https://github.com/odespesse/web-bloom/blob/master/LICENSE)).

## Resources

- A good reference to start with Rust + WASM: [Getting started with WebAssembly and Rust](https://blog.logrocket.com/getting-started-with-webassembly-and-rust/)
- Building and working with rust-generated WebAssembly: [wasm-pack](https://github.com/rustwasm/wasm-pack)

https://www.gutenberg.org/ebooks/author/65
