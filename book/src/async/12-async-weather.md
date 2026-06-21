# Mini Project 1 — Async Weather

You built a parallel weather fetcher with threads in the threading chapter. Now rebuild it with async. The goal is not to do something new — it is to see exactly what changes, and what stays the same.

```toml
[dependencies]
tokio    = { version = "1", features = ["full"] }
reqwest  = { version = "0.12", features = ["json"] }
serde_json = "1"
```

---

## What we built before

The threaded version (from "Mini Project 1 — Parallel Weather") did this:

```rust
// threaded version
let (tx, rx) = mpsc::channel::<(String, Result<f64, String>)>();

for (name, lat, lon) in cities {
    let tx = tx.clone();
    thread::spawn(move || {
        let result = fetch_temp(name, lat, lon);   // blocking reqwest
        tx.send((name.to_string(), result)).unwrap();
    });
}
drop(tx);

for (city, result) in rx { ... }
```

One OS thread per city. Six threads running simultaneously, each blocking on its HTTP request. Total time: as long as the slowest request.

---

## What changes in the async version

| | Threaded | Async |
|---|---|---|
| Import | `use std::thread` | `use tokio::...` |
| Spawn | `thread::spawn(move \|\| { ... })` | `tokio::spawn(async move { ... })` |
| Wait for result | `handle.join().unwrap()` | `handle.await.unwrap()` |
| HTTP client | `reqwest::blocking::Client` | `reqwest::Client` (async) |
| Channel | `std::sync::mpsc` | `tokio::sync::mpsc` |
| OS threads used | 1 per city (6 total) | 1 per CPU core (tokio's pool) |

The structure is almost identical. The types change. The async version uses far fewer OS threads — all six fetches run on Tokio's thread pool, typically one thread per core.

---

## Step 1 — The cities

Same six cities as the threading chapter:

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

---

## Step 2 — The async fetch function

The threaded version used `reqwest::blocking::Client`. Async code uses the non-blocking `reqwest::Client` instead — same API shape, different type:

```rust
use reqwest::Client;
use serde_json::Value;

async fn fetch_temp(
    client: &Client,
    city: &str,
    lat: f64,
    lon: f64,
) -> Result<f64, String> {
    let url = format!(
        "https://api.open-meteo.com/v1/forecast\
         ?latitude={lat}&longitude={lon}&current_weather=true"
    );

    let resp: Value = client
        .get(&url)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json()
        .await
        .map_err(|e| e.to_string())?;

    resp["current_weather"]["temperature"]
        .as_f64()
        .ok_or_else(|| "missing temperature field".to_string())
}
```

Two differences from the threading version:

1. The function is `async fn` — it returns a future.
2. Both `.send()` and `.json()` need `.await` — they are async operations that yield to the runtime while waiting for the network.

`reqwest::Client` is cheap to clone and designed to be reused across many requests (it manages a connection pool internally). We create one client and pass a reference to each task.

---

## Step 3 — Spawn one task per city

```rust
let client = Client::new();

let handles: Vec<_> = cities.iter().map(|(name, lat, lon)| {
    let client = client.clone();
    let name = name.to_string();
    tokio::spawn(async move {
        let result = fetch_temp(&client, &name, *lat, *lon).await;
        (name, result)
    })
}).collect();
```

Compare with the threaded version:

```rust
// threaded
thread::spawn(move || {
    let result = fetch_temp(name, lat, lon);   // blocking
    tx.send((name.to_string(), result)).unwrap();
});

// async
tokio::spawn(async move {
    let result = fetch_temp(&client, &name, *lat, *lon).await;  // non-blocking
    (name, result)
})
```

The shape is the same. `move ||` becomes `async move`. The blocking call gets `.await`. The return type travels through the `JoinHandle` instead of a channel — we return the result directly from the task closure.

---

## Step 4 — Collect results

```rust
for handle in handles {
    match handle.await.unwrap() {
        (city, Ok(temp)) => println!("{city:<12}  {temp:>6.1}°C"),
        (city, Err(e))   => println!("{city:<12}  error: {e}"),
    }
}
```

Compare with the threaded version, which iterated over `rx` (the channel). Here we iterate over `handles` and `.await` each one. The order is the submission order — we collect handles in city order and await them in that order, so results print city by city regardless of which fetch finished first.

If you want results in arrival order (like the threaded version), use a channel here too — `tokio::sync::mpsc` with the same sender/receiver pattern.

---

## Full program

```rust
use reqwest::Client;
use serde_json::Value;
use std::time::Instant;

async fn fetch_temp(
    client: &Client,
    city: &str,
    lat: f64,
    lon: f64,
) -> Result<f64, String> {
    let url = format!(
        "https://api.open-meteo.com/v1/forecast\
         ?latitude={lat}&longitude={lon}&current_weather=true"
    );

    let resp: Value = client
        .get(&url)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json()
        .await
        .map_err(|e| e.to_string())?;

    resp["current_weather"]["temperature"]
        .as_f64()
        .ok_or_else(|| "missing temperature field".to_string())
}

#[tokio::main]
async fn main() {
    let cities = vec![
        ("Tehran",    35.6892,  51.3890),
        ("London",    51.5074,  -0.1278),
        ("New York",  40.7128, -74.0060),
        ("Tokyo",     35.6762, 139.6503),
        ("Sydney",   -33.8688, 151.2093),
        ("Paris",     48.8566,   2.3522),
    ];

    let client = Client::new();
    let t0 = Instant::now();

    let handles: Vec<_> = cities.iter().map(|(name, lat, lon)| {
        let client = client.clone();
        let name = name.to_string();
        let (lat, lon) = (*lat, *lon);
        tokio::spawn(async move {
            let result = fetch_temp(&client, &name, lat, lon).await;
            (name, result)
        })
    }).collect();

    println!("{:<12}  {}", "City", "Temperature");
    println!("{}", "─".repeat(24));

    for handle in handles {
        match handle.await.unwrap() {
            (city, Ok(temp)) => println!("{city:<12}  {temp:>6.1}°C"),
            (city, Err(e))   => println!("{city:<12}  error: {e}"),
        }
    }

    println!("\nfetched in {:.2?}", t0.elapsed());
}
```

---

## The thread count difference

Run both versions and check the thread count (on macOS: `ps -M <pid>`; on Linux: `ls /proc/<pid>/task | wc -l`).

```
threaded version:   ~8 threads  (main + 6 fetch threads + 1 tokio internal)
async version:      ~4 threads  (tokio's pool: one per core, on a 4-core machine)
```

With six cities the difference is small. Repeat the list 100 times (600 cities) and the async version still uses ~4 threads while the threaded version tries to spawn 600. The async version does not slow down as the number of tasks grows; the threaded version does.

---

## Discussion questions

These are worth thinking through — there is no single right answer.

**What happens if you await each handle one at a time in a for loop instead of collecting first?**

All six tasks are still spawned immediately (in the `map(...).collect()` step), so they run concurrently. Awaiting handles one by one only affects the order you see the results — it does not serialise the fetches. The total time is still dominated by the slowest single request.

**How would you add a 3-second timeout to each fetch?**

Wrap the `fetch_temp` call in `tokio::time::timeout`:

```rust
use tokio::time::{timeout, Duration};

let result = timeout(
    Duration::from_secs(3),
    fetch_temp(&client, &name, lat, lon),
).await;

match result {
    Ok(inner)  => inner,             // fetch_temp's Result<f64, String>
    Err(_)     => Err("timed out".to_string()),
}
```

**Could you use `tokio::join!` directly instead of spawning tasks? What is the trade-off?**

Yes. `join!` runs futures concurrently inside a single task. `tokio::spawn` puts each future in its own task, allowing the runtime to schedule them across threads.

For six HTTP requests, `join!` is fine — the bottleneck is network latency, not CPU, so running on one thread is not a problem. The practical difference only matters when the work is CPU-intensive, or when you want one failure to not affect the others (spawned tasks can fail independently; `join!` propagates panics).

---

## Key APIs

| API | What it does |
|-----|-------------|
| `reqwest::Client::new()` | Async HTTP client — clone cheaply, reuse across tasks |
| `.send().await` | Send the request; yields while waiting for the response |
| `.json::<Value>().await` | Deserialise response body as JSON; yields while reading |
| `resp["key"].as_f64()` | Extract a JSON number field |
| `tokio::spawn(async move { ... })` | Spawn a task; returns a `JoinHandle` |
| `handle.await.unwrap()` | Wait for a spawned task to finish and get its return value |
| `Instant::now()` / `.elapsed()` | Measure wall-clock time |
