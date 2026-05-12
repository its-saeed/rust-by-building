// Exercise 5.3 — loop with break
//
// This loop runs forever. Add a `break` so it stops after
// printing "done" when the counter reaches 3.
//
// Expected output:
//   step 1
//   step 2
//   step 3
//   done

fn main() {
    let mut n = 1;
    loop {
        println!("step {}", n);
        if n == 3 {
            println!("done");
            // TODO: add break here
        }
        n += 1;
    }
}

// No assertions — if it prints the expected output and exits, you've solved it.
