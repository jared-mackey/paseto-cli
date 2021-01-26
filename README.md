# paseto-cli

A simple CLI that can extract unverified public paseto v2 token data. Handy for quickly inspecting the payload of a token without needing the public keys.

# Usage

```bash
$ git clone git@github.com:jared-mackey/paseto-cli.git
$ cargo install --path paseto-cli
$ paseto-cli token_goes_here
# Or if you have a token in your clipboard
$ pbpaste | paseto-cli
```
