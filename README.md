# Jupiter api rust example

We can autogenerate a client with openapi-gen and the openapi schema but the structs cannot be directly consumed as solana sdk or appropirate rust types.

The crate `jupiter-swap-api` exposes the API types to allow

If you must use solana 1.16 crates, there is a work in progress to relax tokio pinned too low conflicting with many other crates https://github.com/solana-labs/solana/pull/32943

### Using hosted swap API URLs

You can set custom URLs via environment variables for any self-hosted Jupiter APIs. Like the [V6 Swap API](https://station.jup.ag/docs/apis/self-hosted) or [QuickNode's Metis API](https://marketplace.quicknode.com/add-on/metis-jupiter-v6-swap-api). Here are the ENV vars:

```
API_BASE_URL=https://jupiter-swaps.quiknode.pro/D3ADB33F
```
