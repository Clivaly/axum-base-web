## After Upate:

```sh
# Terminal 1 - To run the server:
$ cargo watch -q -c -w src/ -x run
$ cargo watch -q -c -w src/ -w .cargo/ -x "run"

# Terminal 2 - To run the quick_dev:
$ cargo watch -q -c -w examples/ -x "run --examples"
```

## Before Upate:
```sh
# Terminal 1 - To run the server:
$ cargo watch -q -c -w src/ -x run

# Terminal 2- To run the test quick_dev:
$ cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"
```