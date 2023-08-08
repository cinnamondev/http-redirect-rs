# http-redirect-rs

![Proof of concept showing fake url that resembles url format uname:passwd@domain](./.assets/concept-demo.gif)

Proof of concept for redirecting uris using the `username:password@domain` schema to the real page after downloading a payload.

## Potential use case

A malicious attacker could in theory provide a uri to a real download page and download a mimick zip file. The inspiration for this was the introduction of the .zip tld, as a malicious attacker could easily make a webpage with domain, say, `release.zip` so the user would be expecting a download.

This has probably been done a couple times already, but i thought it was an interesting idea at the time. As shown in the GIF, certain browsers (firefox notably) will prevent immediate "log-in", which may alert the user to trouble. Firefox will not display a period in the uri normally so a comma was used in the username instead.

In chrome, however, it passes the username:password but strips it from the uri without any user input. This is much more widely used and likely to be a target more so.

## Unicode replacements

```rust
fn map_special(c: char) -> char {
    match c {
        '∕' => '/', // U+2215 (Division Slash) to /
        ',' => '.',
        c => c,
    }
}
```

Special unicode character `∕` can be used in the `username:password` section, but `/` cannot. This can trick users into thinking they are clicking a real url, though those who are more vigilant will notice the oddly formatted symbol.

## License & Notice

This code is licensed under the Apache 2.0 license and bears no malicious intent. It is a proof of concept, and others (ie security researchers and or malicious attackers) could certaintly make up something more eloquent or with less tell-tale signs (ie the period between a download and final redirect looks odd as it currently is.)
