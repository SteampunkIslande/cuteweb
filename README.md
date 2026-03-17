# Security

A default `Rocket.toml` comes with the binary. It contains sensible defaults but **it is only meant for local execution**!

These are the two configurations that must be defined in `Rocket.toml` if you're running on a shared HTTP server:

```toml
local = true
secret_key = "sTooPaHkmo2i/cXXwawBf4per11UBB1x7nRJvUxPfFg="
```

This secret key is not really secret now, but it doesn't really matter if the app is only served on localhost. If you're running this app on a shared HTTP server, this key must be kept secret, as it ensures user's authentication and thus access to all the data.

There is also the `local = true` parameter, that basically disables user authentication, and makes every session's user the user with ID=0.
