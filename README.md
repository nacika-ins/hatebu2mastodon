# hatebu2mastodon

**alpha release version**

## SETUP

1. Please rewrite the maston URL of `send_mstdn.rs`.

```
let mut registration = Registration::new("https://mstdn.nacika.com");
```

2. Enter the Webhook api key of Hatena bookmark in config.toml

3. Please execute by cargo run and perform authentication. It must be started multiple times.
