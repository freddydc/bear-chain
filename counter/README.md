# Counter

## Getting Started

Install cli:

```bash
npm install -g near-cli
```

Login to wallet:

```bash
near login
```

Build:

```bash
cargo build --target wasm32-unknown-unknown --release
```

Deploy:

```bash
near deploy --accountId <YOUR_ACCOUNT_HERE> --wasmFile target/wasm32-unknown-unknown/release/<NAME_OF_YOUR_PROJECT>.wasm
```

Just run:

```bash
# <NAME_OF_FUNCTION>
# get_result
# add
# subtract
# reset
near call <YOUR_ACCOUNT_HERE> <NAME_OF_FUNCTION> --accountId <YOUR_ACCOUNT_HERE>
```
