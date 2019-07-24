# Wasm-powered QR code image generator

This library exposes two functions, typed accordingly:

```ts
// Encode binary data as PNG QR code
export function binary(data: Uint8Array): string | undefined;

// Encode string as PNG QR code
export function string(data: string): string | undefined;
```

The return value in both cases is a [data URI](https://en.wikipedia.org/wiki/Data_URI_scheme) string, ready to be used as a source for any image.
