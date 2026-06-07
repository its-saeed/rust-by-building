# Chapter 6 — HTTP and the Web

You have been using HTTP your entire programming life — every time a browser loads a page, every time you call a web API. HTTP is the application-layer protocol of the web. It runs over TCP, which handles delivery, so HTTP can focus entirely on the conversation.

> **Think of it like this:** HTTP works like ordering at a restaurant. You (the client) call the waiter (HTTP) over and place your order (the request): the method is what you want done ("bring me"), the path is the item ("/pasta/carbonara"), and headers are special instructions ("no dairy, please"). The waiter disappears into the kitchen and comes back once with your food (the response): a status code tells you if it went well (200 OK — here's your pasta) or not (404 Not Found — we don't have that dish, 500 Internal Server Error — the kitchen is on fire). Crucially, the waiter never brings you something you didn't order. And you get exactly one response per order — not a stream, not a subscription.

---

## Request and response

HTTP is a **request-response** protocol. The client sends a request; the server sends exactly one response. The server never sends anything unprompted.

Every HTTP request has:

- A **method** — what to do (`GET`, `POST`, `PUT`, `DELETE`)
- A **path** — which resource (`/weather?city=berlin`)
- **Headers** — metadata (`Content-Type: application/json`)
- Optionally a **body** — data to send (for POST/PUT)

Every response has:

- A **status code** — the outcome (200 OK, 404 Not Found, 500 Internal Server Error)
- **Headers** — metadata about the response
- Optionally a **body** — the returned data

---

## What it looks like on the wire

HTTP is plain text (for HTTP/1.1). If you could read the bytes flowing over a TCP connection, a request looks like:

```
GET /forecast?latitude=52.52&longitude=13.41 HTTP/1.1\r\n
Host: api.open-meteo.com\r\n
\r\n
```

And the response starts with:

```
HTTP/1.1 200 OK\r\n
Content-Type: application/json\r\n
\r\n
{"latitude":52.52,"longitude":13.41,...}
```

Headers are separated from the body by a blank line (`\r\n\r\n`). The body follows immediately after.

You will never write HTTP at this level in Rust — libraries handle formatting and parsing. But knowing the structure helps when you read documentation or debug unexpected responses.

---

## Status codes

The three-digit status code tells you everything about the outcome:

| Range | Meaning | Examples |
|-------|---------|---------|
| 2xx | Success | 200 OK, 201 Created |
| 3xx | Redirect | 301 Moved Permanently, 302 Found |
| 4xx | Client error | 400 Bad Request, 404 Not Found, 401 Unauthorized |
| 5xx | Server error | 500 Internal Server Error, 503 Service Unavailable |

A 4xx means *you* did something wrong. A 5xx means *the server* did something wrong. A 2xx means the request succeeded.

---

## JSON — the data format of APIs

Most modern web APIs speak **JSON** (JavaScript Object Notation) in their request and response bodies. JSON encodes structured data as human-readable text:

```json
{
  "city": "Berlin",
  "temperature": 18.4,
  "conditions": ["cloudy", "windy"],
  "forecast": {
    "tomorrow": 15.1
  }
}
```

In Rust, the `serde_json` crate parses and generates JSON. Combined with `#[derive(Deserialize)]`, you can map JSON directly to Rust structs without writing any parsing code by hand.

---

## Terminal — `curl`

`curl` is the command-line HTTP client. It speaks HTTP exactly as a browser would.

A basic GET request:

```sh
curl https://api.open-meteo.com/v1/forecast?latitude=52.52&longitude=13.41&current=temperature_2m
```

Add `-i` to see the response headers:

```sh
curl -i https://httpbin.org/get
```

Add `-v` to see the full conversation — request headers, response headers, everything:

```sh
curl -v https://httpbin.org/get
```

`httpbin.org` is a testing service that echoes back what you sent. It is useful for exploring HTTP before connecting to real APIs.

---

## Terminal — POST with curl

```sh
curl -X POST https://httpbin.org/post \
  -H "Content-Type: application/json" \
  -d '{"name": "Alice", "age": 30}'
```

- `-X POST` — use the POST method
- `-H` — add a header
- `-d` — the request body

The response will echo back your JSON so you can verify it arrived correctly.

---

## Free APIs worth exploring

All of these require no sign-up and no API key (or accept a free `DEMO_KEY`). Try them with `curl` before writing Rust code — that is how real API development works.

### NASA Astronomy Picture of the Day

```sh
curl "https://api.nasa.gov/planetary/apod?api_key=DEMO_KEY"
```

Returns the title, explanation, and URL of today's astronomy photo. The `DEMO_KEY` allows 30 requests per hour. The response has a `url` field pointing to the actual image.

### Open Trivia Database

```sh
curl "https://opentdb.com/api.php?amount=5&type=multiple&category=18"
```

Returns 5 multiple-choice questions (category 18 = computers). Each question has a `correct_answer` and three `incorrect_answers`. Great for building a quiz game.

### Chuck Norris jokes

```sh
curl "https://api.chucknorris.io/jokes/random"
```

Returns a single joke in `{"value": "..."}`. The simplest possible JSON response — good for a first serde exercise. You can also filter by category: `?category=dev`.

### JokeAPI — programming jokes

```sh
curl "https://v2.jokeapi.dev/joke/Programming?type=single"
```

Returns a programming-specific joke. The `type=single` gives you one-liners. Without it you get two-part setup/punchline jokes with separate `setup` and `delivery` fields.

### REST Countries

```sh
curl "https://restcountries.com/v3.1/name/germany"
```

Returns detailed information about a country: population, area, capital, currencies, languages, flag emoji. Try replacing `germany` with any country name. A rich response — good for practising selective deserialisation (you do not have to map every field, just the ones you want).

### Your IP's location

```sh
curl "https://ipapi.co/json/"
```

Returns your public IP address and its estimated geographic location (city, country, coordinates, timezone). Every student gets a different response — it is personalised to their connection.

---

## What to carry forward

- HTTP is **request-response**: one request, one response, nothing unprompted
- Requests have a **method + path + headers + optional body**
- Responses have a **status code + headers + optional body**
- Status codes: 2xx = success, 4xx = client error, 5xx = server error
- **JSON** is the standard body format for web APIs; Rust handles it with `serde` and `serde_json`
- `curl` is your terminal HTTP client; `-i` for headers, `-v` for full debug output

The next chapter covers **sockets** — the OS-level abstraction that all of this is built on, and the actual interface Rust gives you to the network.
