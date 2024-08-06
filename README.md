### Auto Run for /src:
* cargo watch -q -c -w src/ -x run

### Auto Run for /tests:
* cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"