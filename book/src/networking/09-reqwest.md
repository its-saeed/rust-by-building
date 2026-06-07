# Lesson 2 — HTTP Client with reqwest

The standard library handles TCP and UDP, but HTTP is more complex: you need to format request headers, parse status codes, handle redirects, and decode response bodies. The `reqwest` crate does all of that for you.

In this lesson you will call a real weather API and deserialise the JSON response into a Rust struct.

---

## Project setup

```sh
rbb start
```

Add to `Cargo.toml`:

```toml
[dependencies]
reqwest = { version = "0.12", features = ["blocking", "json"] }
serde = { version = "1", features = ["derive"] }
```

- `blocking` gives you a synchronous API — no async/await required
- `json` enables `.json::<T>()` for deserialising responses directly into Rust types
- `serde` with `derive` gives you `#[derive(Deserialize)]`

---

## The API

We will use [Open-Meteo](https://open-meteo.com/) — a free weather API that requires no API key. A request looks like:

```
https://api.open-meteo.com/v1/forecast
  ?latitude=52.52
  &longitude=13.41
  &current=temperature_2m,wind_speed_10m
```

Try it in your browser or with `curl`:

```sh
curl "https://api.open-meteo.com/v1/forecast?latitude=52.52&longitude=13.41&current=temperature_2m,wind_speed_10m"
```

The response is JSON. Look at the `current` field — that is what we will deserialise.

---

## Step 1 — Define the response types

```rust
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Response {
    latitude:  f64,
    longitude: f64,
    current:   Current,
}

#[derive(Deserialize, Debug)]
struct Current {
    temperature_2m:  f64,
    wind_speed_10m:  f64,
}
```

`#[derive(Deserialize)]` generates code that maps JSON keys to struct fields. Field names must match the JSON keys exactly.

---

## Step 2 — Make the request

```rust
use reqwest::blocking::Client;

fn main() {
    let client = Client::new();

    let response: Response = client
        .get("https://api.open-meteo.com/v1/forecast")
        .query(&[
            ("latitude",  "52.52"),
            ("longitude", "13.41"),
            ("current",   "temperature_2m,wind_speed_10m"),
        ])
        .send()
        .expect("request failed")
        .json()
        .expect("failed to parse JSON");

    println!("Berlin weather:");
    println!("  Temperature: {:.1}°C", response.current.temperature_2m);
    println!("  Wind speed:  {:.1} km/h", response.current.wind_speed_10m);
}
```

`.query()` appends query parameters to the URL. `.send()` performs the HTTP request. `.json::<Response>()` reads the response body and deserialises it — this is where `serde` does its work.

---

## Full program

```rust
use reqwest::blocking::Client;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Response {
    latitude:  f64,
    longitude: f64,
    current:   Current,
}

#[derive(Deserialize, Debug)]
struct Current {
    temperature_2m: f64,
    wind_speed_10m: f64,
}

fn main() {
    let client = Client::new();

    let response: Response = client
        .get("https://api.open-meteo.com/v1/forecast")
        .query(&[
            ("latitude",  "52.52"),
            ("longitude", "13.41"),
            ("current",   "temperature_2m,wind_speed_10m"),
        ])
        .send()
        .expect("request failed")
        .json()
        .expect("failed to parse JSON");

    println!("Weather at ({}, {}):", response.latitude, response.longitude);
    println!("  Temperature: {:.1}°C", response.current.temperature_2m);
    println!("  Wind speed:  {:.1} km/h", response.current.wind_speed_10m);
}
```

---

## Exercise

> **TODO 1**: Add a `hourly_units` field to `Response` — look at the raw JSON to see what it contains.
>
> **TODO 2**: Request `hourly=temperature_2m` instead of (or in addition to) `current`. The `hourly` field contains arrays. Add `hourly: Hourly` to `Response` with `struct Hourly { temperature_2m: Vec<f64> }`. Print the first 6 hours.
>
> **TODO 3**: Take latitude and longitude as command-line arguments. Use `std::env::args()` to read them. Test with your own city's coordinates (search "[city name] coordinates" to find them).

---

## Checking the status code

If you want to handle errors gracefully rather than panicking:

```rust
let resp = client
    .get(url)
    .send()
    .expect("network error");

if !resp.status().is_success() {
    eprintln!("API returned {}", resp.status());
    std::process::exit(1);
}

let data: Response = resp.json().expect("parse error");
```

`.status()` returns a `StatusCode`. `.is_success()` returns true for 2xx codes.

---

## Other APIs to try

Once you have the weather client working, try replacing the API with one of these. Each has a different response shape — good practice for writing `#[derive(Deserialize)]` structs.

### Chuck Norris jokes

```rust
#[derive(Deserialize, Debug)]
struct Joke {
    value: String,
}

let joke: Joke = client
    .get("https://api.chucknorris.io/jokes/random")
    .send()?
    .json()?;

println!("{}", joke.value);
```

### NASA Astronomy Picture of the Day

```rust
#[derive(Deserialize, Debug)]
struct Apod {
    title:       String,
    explanation: String,
    url:         String,
    date:        String,
}

let apod: Apod = client
    .get("https://api.nasa.gov/planetary/apod")
    .query(&[("api_key", "DEMO_KEY")])
    .send()?
    .json()?;

println!("{} ({})", apod.title, apod.date);
println!("{}", apod.url);
```

### Open Trivia Database

```rust
#[derive(Deserialize, Debug)]
struct TriviaResponse {
    results: Vec<Question>,
}

#[derive(Deserialize, Debug)]
struct Question {
    question:         String,
    correct_answer:   String,
    incorrect_answers: Vec<String>,
}

let trivia: TriviaResponse = client
    .get("https://opentdb.com/api.php")
    .query(&[("amount", "3"), ("type", "multiple"), ("category", "18")])
    .send()?
    .json()?;

for q in &trivia.results {
    println!("Q: {}", q.question);
    println!("A: {}", q.correct_answer);
}
```

Note: the trivia API HTML-encodes some characters in strings (e.g. `&amp;` for `&`). You will need to unescape them for a real quiz app, but for the exercise it does not matter.

---

## What reqwest handles for you

When you call `.send()`, reqwest:

1. Resolves the hostname via DNS
2. Opens a TCP connection to the server
3. Formats the HTTP request (method, path, query string, headers)
4. Sends it over the TCP stream
5. Reads the response status and headers
6. Returns a `Response` you can read from

The `.json()` call then reads the response body and passes it through serde's deserialiser. What took dozens of lines of raw TCP code in chapter 4 happens in one method call.

---

## Key APIs

| API | What it does |
|-----|-------------|
| `Client::new()` | Create an HTTP client (reusable, manages connection pooling) |
| `.get(url)` | Start a GET request builder |
| `.query(&[...])` | Append query parameters |
| `.send()` | Execute the request, get a `Response` |
| `.status()` | HTTP status code |
| `.json::<T>()` | Deserialise body as JSON into type T |
| `#[derive(Deserialize)]` | Generate JSON→struct mapping |
