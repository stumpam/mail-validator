# mail-validator
REST app for validating email adresses written in Rust

## How to use the app
After app is running call `GET` on adress `http://localhost:1234/validate_email?email=test@test.com`.

It returns JSON object whether email address is valid or not
```json
{
  "valid": true
}
```

It accepts attribute `-p` or `--port` to setup localhost port where an app listens, default port is *1234*

# Run app
1. clone directory
2. run `cargo run`

# Build app
run `cargo build --release`
