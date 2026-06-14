# Mini Project 1 — Parallel Weather

You have used `thread::spawn` and channels in isolation. Now apply them to a real task: fetch the current temperature for six cities simultaneously, collect the results, and print them in any order they arrive.

This is **I/O-bound parallelism** — each thread is not crunching numbers, it is waiting for a network response. Even on a single CPU core, doing these requests in parallel is faster than sequentially because the CPU is idle while waiting for the network. Threads let other work happen during that wait.

---

## Sequential vs parallel

Sequential — the worst case:

```
time ──────────────────────────────────────────────────────────────────▶
fetch Tehran   [────── 400ms ──────]
fetch London                        [────── 380ms ──────]
fetch New York                                           [── 410ms ──]
...
total: ~2.4 seconds
```

Parallel — all in flight at once:

```
time ──────────────────────────────────────────────────────────────────▶
fetch Tehran   [────── 400ms ──────]
fetch London   [────── 380ms ──────]
fetch New York [── 410ms ──────────]
...
total: ~410ms (the slowest request)
```

The total time becomes the *slowest* single request, not the *sum* of all requests.

---

## The API

We use Open-Meteo — free, no key required, already covered in the networking lessons:

```
https://api.open-meteo.com/v1/forecast
  ?latitude=35.69
  &longitude=51.39
  &current_weather=true
```

Returns JSON like:
```json
{
  "current_weather": {
    "temperature": 22.4,
    "windspeed": 12.0,
    ...
  }
}
```

---

## Step 1 — The cities

```rust
let cities = vec![
    ("Tehran",    35.6892,  51.3890),
    ("London",    51.5074,  -0.1278),
    ("New York",  40.7128, -74.0060),
    ("Tokyo",     35.6762, 139.6503),
    ("Sydney",   -33.8688, 151.2093),
    ("Paris",     48.8566,   2.3522),
];
```

Each entry is `(name, latitude, longitude)`.

---

## Step 2 — The fetch function

Each thread calls this. It creates its own `reqwest::blocking::Client`, makes the request, and extracts the temperature:

```rust
use reqwest::blocking::Client;
use serde_json::Value;

fn fetch_temp(city: &str, lat: f64, lon: f64) -> Result<f64, String> {
    let url = format!(
        "https://api.open-meteo.com/v1/forecast\
         ?latitude={lat}&longitude={lon}&current_weather=true"
    );

    let resp: Value = Client::new()
        .get(&url)
        .send()
        .map_err(|e| e.to_string())?
        .json()
        .map_err(|e| e.to_string())?;

    resp["current_weather"]["temperature"]
        .as_f64()
        .ok_or_else(|| "missing temperature field".to_string())
}
```

`map_err(|e| e.to_string())` converts the various error types to `String` so the channel can carry a uniform `Result<f64, String>`.

---

## Step 3 — Spawn one thread per city

Each thread gets a clone of `tx` and ownership of the city data:

```rust
use std::sync::mpsc;
use std::thread;

let (tx, rx) = mpsc::channel::<(String, Result<f64, String>)>();

for (name, lat, lon) in cities {
    let tx = tx.clone();
    thread::spawn(move || {
        let result = fetch_temp(name, lat, lon);
        tx.send((name.to_string(), result)).unwrap();
    });
}

drop(tx);
```

All six threads start immediately. They run in parallel — each makes its HTTP request independently. As each finishes, it sends its result through the channel.

`drop(tx)` closes the original sender so the receiver knows when all threads are done.

---

## Step 4 — Collect results

The main thread receives results as they arrive — not necessarily in the order the threads were spawned:

```rust
for (city, result) in rx {
    match result {
        Ok(temp) => println!("{city:12} {temp:>6.1}°C"),
        Err(e)   => println!("{city:12} error: {e}"),
    }
}
```

The city that responds fastest prints first. You will rarely see the same order twice.

---

## Full program

```rust
use reqwest::blocking::Client;
use serde_json::Value;
use std::sync::mpsc;
use std::thread;
use std::time::Instant;

fn fetch_temp(city: &str, lat: f64, lon: f64) -> Result<f64, String> {
    let url = format!(
        "https://api.open-meteo.com/v1/forecast\
         ?latitude={lat}&longitude={lon}&current_weather=true"
    );
    let resp: Value = Client::new()
        .get(&url)
        .send()
        .map_err(|e| e.to_string())?
        .json()
        .map_err(|e| e.to_string())?;
    resp["current_weather"]["temperature"]
        .as_f64()
        .ok_or_else(|| "missing temperature field".to_string())
}

fn main() {
    let cities = vec![
        ("Tehran",    35.6892,  51.3890),
        ("London",    51.5074,  -0.1278),
        ("New York",  40.7128, -74.0060),
        ("Tokyo",     35.6762, 139.6503),
        ("Sydney",   -33.8688, 151.2093),
        ("Paris",     48.8566,   2.3522),
    ];

    let (tx, rx) = mpsc::channel::<(String, Result<f64, String>)>();

    let t0 = Instant::now();

    for (name, lat, lon) in cities {
        let tx = tx.clone();
        thread::spawn(move || {
            let result = fetch_temp(name, lat, lon);
            tx.send((name.to_string(), result)).unwrap();
        });
    }
    drop(tx);

    println!("{:<12}  {}", "City", "Temperature");
    println!("{}", "─".repeat(24));

    for (city, result) in rx {
        match result {
            Ok(temp) => println!("{city:<12}  {temp:>6.1}°C"),
            Err(e)   => println!("{city:<12}  error: {e}"),
        }
    }

    println!("\nfetched in {:.2?}", t0.elapsed());
}
```

---

## What to observe

Run it a few times. Notice:
- The order of results changes on every run — whichever city responds fastest arrives first
- The total time is close to the slowest single request, not the sum
- If you lose network connectivity, all six errors arrive almost simultaneously

Now try commenting out the threads and making the calls sequentially in a loop. Time both versions. The difference is the entire point of I/O-bound parallelism.

---

## Why each thread creates its own Client

`reqwest::blocking::Client` manages a connection pool internally. Creating one per thread is fine and safe — the `Client` is `Send`, so it can be moved into a thread. In a production program you might share one client with `Arc` to reuse connections, but for six requests the cost is negligible.

---

## Exercise

> **TODO 1**: Add the `windspeed` field alongside temperature. The API returns it in the same `current_weather` object.
>
> **TODO 2**: Measure the sequential version: remove the threads and call `fetch_temp` in a plain `for` loop. Print the time for both and compute the speedup ratio.
>
> **TODO 3**: What happens if you spawn 100 threads (add more cities, or repeat the list)? Does it keep getting faster? At what point does spawning more threads stop helping and why?

---

## Key APIs

| API | What it does |
|-----|-------------|
| `reqwest::blocking::Client::new()` | HTTP client — safe to create per thread |
| `.send().map_err(\|e\| e.to_string())` | Make request, convert error to String |
| `.json::<Value>()` | Deserialise response body as JSON |
| `resp["key"].as_f64()` | Extract a JSON number field |
| `Instant::now()` / `.elapsed()` | Measure wall-clock time |
