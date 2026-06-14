# Mini Project — HTTP over Raw TCP

HTTP is not magic. It is plain text sent over a TCP connection. This project makes that concrete: first you will speak HTTP by hand using `nc`, then write the same conversation in Rust using nothing but `TcpStream`.

---

## What a browser actually does

When you type `http://example.com` into a browser, the browser:

1. Resolves `example.com` to an IP address (DNS)
2. Opens a TCP connection to port 80
3. Sends a text message following the HTTP format
4. Reads the server's text response
5. Parses and renders the HTML

Steps 3 and 4 are exactly what you will do by hand.

---

## The HTTP request format

An HTTP GET request is three parts, separated by `\r\n` (carriage return + newline):

```
GET / HTTP/1.0\r\n          ← request line: method, path, version
Host: example.com\r\n       ← header
\r\n                        ← blank line signals "end of headers"
```

That is the entire request. No binary encoding, no framing, just text.

We use **HTTP/1.0** rather than 1.1 because 1.0 closes the connection after sending the response — simpler to read since you just read until EOF. HTTP/1.1 uses keep-alive and requires parsing `Content-Length` to know when the response ends.

---

## Part 1 — With `nc`

`nc` (netcat) opens a raw TCP connection and lets you type into it. It is a direct window into what the protocol looks like.

```sh
nc example.com 80
```

Once connected (no output — it just waits), type the request **exactly** as shown, then press Enter twice (the blank line signals end of headers):

```
GET / HTTP/1.0
Host: example.com

```

The server responds immediately:

```
HTTP/1.0 200 OK
Content-Type: text/html; charset=UTF-8
Content-Length: 1256
...

<!doctype html>
<html>
<head>
    <title>Example Domain</title>
...
```

Two sections separated by a blank line: **headers** (metadata) then **body** (the actual HTML). The connection closes when the response is fully sent.

What you just did is exactly what a browser does — you simply typed the request manually instead of having software generate it.

---

## Part 2 — In Rust with `TcpStream`

Now write the same conversation in code:

```rust
use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    // 1. open a TCP connection to port 80
    let mut stream = TcpStream::connect("example.com:80")
        .expect("connection failed");

    // 2. send the HTTP request
    let request = "GET / HTTP/1.0\r\nHost: example.com\r\n\r\n";
    stream.write_all(request.as_bytes())
        .expect("write failed");

    // 3. read the full response
    let mut response = String::new();
    stream.read_to_string(&mut response)
        .expect("read failed");

    println!("{response}");
}
```

Run it:

```sh
cargo run
```

You get the same response you saw with `nc` — headers first, blank line, then the HTML body.

---

## What each line does

**`TcpStream::connect("example.com:80")`**
The standard library resolves the hostname via DNS and opens the TCP connection — the same two steps the browser performs. Port 80 is the conventional HTTP port.

**`"GET / HTTP/1.0\r\nHost: example.com\r\n\r\n"`**
The `\r\n` pairs are the CRLF line endings the HTTP specification requires. The double `\r\n` at the end is the blank line that tells the server "headers are done, respond now."

**`write_all`**
Sends all bytes of the request string. Plain `write` might only send part of the buffer (the OS can split it); `write_all` loops until every byte is delivered.

**`read_to_string`**
Reads until the server closes the connection (EOF). With HTTP/1.0 the server closes after sending the full response, so this collects the entire thing.

---

## Separating headers from body

The response mixes headers and HTML. Split on the blank line to handle them separately:

```rust
if let Some((headers, body)) = response.split_once("\r\n\r\n") {
    println!("=== HEADERS ===");
    println!("{headers}");
    println!("\n=== BODY ===");
    println!("{body}");
}
```

`split_once` splits at the first occurrence of the delimiter and returns `Some((left, right))`. If the delimiter is not found it returns `None`.

---

## Parsing the status code

The first line of the response is the status line:

```
HTTP/1.0 200 OK
```

Extract the status code:

```rust
let status_line = response.lines().next().unwrap_or("");
let status_code = status_line.split_whitespace().nth(1).unwrap_or("?");
println!("status: {status_code}");
```

`split_whitespace()` splits on any whitespace; `.nth(1)` takes the second token — the numeric code.

---

## Full program

```rust
use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    let host = "example.com";

    let mut stream = TcpStream::connect((host, 80))
        .expect("connection failed");

    let request = format!("GET / HTTP/1.0\r\nHost: {host}\r\n\r\n");
    stream.write_all(request.as_bytes()).expect("write failed");

    let mut response = String::new();
    stream.read_to_string(&mut response).expect("read failed");

    let (headers, body) = response
        .split_once("\r\n\r\n")
        .unwrap_or((&response, ""));

    let status = headers.lines().next().unwrap_or("");
    println!("status:  {status}");
    println!("headers: {} lines", headers.lines().count() - 1);
    println!("body:    {} bytes\n", body.len());
    println!("{body}");
}
```

---

## Compare with reqwest

The reqwest version does the same thing in two lines:

```rust
let body = reqwest::blocking::get("http://example.com")
    .unwrap()
    .text()
    .unwrap();
```

Reqwest handles: DNS resolution, TCP connection, HTTP formatting, header parsing, status checking, redirects, TLS, connection pooling, and more. The raw TCP version handles none of that — but it shows exactly what reqwest is doing underneath, which is what this project was for.

---

## Why this does not work for most modern sites

Try replacing `example.com` with `github.com` or `google.com`:

```rust
TcpStream::connect("github.com:80")
```

You will either get a redirect response (`301 Moved Permanently`) pointing to `https://` or a refused connection. Almost every site now requires **HTTPS** — HTTP encrypted with TLS. Plain HTTP on port 80 is increasingly rare.

TLS adds a cryptographic handshake step before the HTTP request. Implementing TLS from scratch is a substantial project; in practice you use a crate like `rustls`. What you have built here is exactly how HTTPS works, minus that encryption layer.

---

## Exercise

> **TODO 1**: Try fetching `http://neverssl.com` — a site that intentionally never redirects to HTTPS, designed for testing exactly this. Does it work?
>
> **TODO 2**: Change the path. Instead of `GET / HTTP/1.0`, try `GET /nonexistent HTTP/1.0`. What status code comes back? What does the body say?
>
> **TODO 3**: Print each response header on its own line, formatted as `name: value`. Headers follow the first line and end at the blank line. Split each on `: ` (colon space).

---

## Key APIs

| API | What it does |
|-----|-------------|
| `TcpStream::connect((host, port))` | DNS resolve + TCP connect in one call |
| `stream.write_all(bytes)` | Send all bytes, looping if needed |
| `stream.read_to_string(&mut s)` | Read until EOF into a String |
| `s.split_once(delimiter)` | Split on the first occurrence of a substring |
| `s.lines().next()` | First line of a multi-line string |
| `s.split_whitespace().nth(n)` | The nth whitespace-separated token |
