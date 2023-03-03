# @remix-pwa/client

A standalone, WASM-built module for the `client` functionalities of [remix-pwa](https://github.com/remix-pwa/remix-pwa). Built with extensibility and speed in mind

## Features

- [x] 100% Test coverage
- [x] Contains support for native browser APIs
- [x] Built with WASM (Rust 🦀)
- [x] Can be used without `remix-pwa`

## Installation

```sh
npm install @remix-pwa/client
```

> _This module requires [`remix-wasm-plugin`](https://github.com/remix-pwa/remix-wasm-plugin) to work with remix_

Add `@remix-pwa/client` to your `wasm.config.js` file. Import the `init` function from `/wasm/@remix-pwa/client/remix_pwa_client` and `wasm` from `/wasm/@remix-pwa/client/remix_pwa_client_bg.wasm`.

```tsx
// entry.client.tsx
import init from "/wasm/@remix-pwa/client/remix_pwa_client";
import wasm from "/wasm/@remix-pwa/client/remix_pwa_client_bg.wasm";

import { initWithProps } from "remix-wasm-plugin";

// hydrate() function
startTransition(() => {
  initWithProps([init, wasm]).then(() => {
    hydrateRoot(document, <RemixBrowser />);
  });
});
```

When you run `npm run dev`, the wasm app would be bundled successfully into the `public` folder. 

## API

_To be implemented..._

## FAQ

<details>
<summary><b>Are there going to be other standalone packages for working with Service Workers, etc?</b></summary>
Oh, yes! But they would probably take time to roll out. I am working on separating them so devs can choose just what they want and finetune it as they
see fit 
</details>

<details>
<summary><b>Why a separate package?</b></summary>
I get questions from devs asking me about how they can use remix-pwa as a template to build their own PWA 
with Remix. My vision for this is that remix-pwa would still be available for people that want to scaffold a 
PWA and not worry about boring details and these packages provide APIs that allow you to finetune your own PWA experience from 
the ground up. 
</details>

<details>
<summary><b>Would <code>remix-pwa</code> still be maintained?</b></summary>
Yes. remix-pwa would still be actively maintained.
</details>

<details>
<summary><b>Why did you make remix-pwa an organisation?</b></summary>
Because it allows for better maintenance and contribution in my opinion. Keeping all related projects in
one place makes things easier to deal with, plus with an org., you can add members and contributors and that
would assist me with maintenance and building a lot.
</details>

<details>
<summary><b>Why Rust though? Why WASM and not JavaScript?</b></summary>
Trying out new things in the Remix ecosystem. I've wanted support for wasm in Remix for a while now
and whilst there are PRs trying to address that, the Remix team can't afford to maintain a whole bunch of 
plugins and features in the main package. Offloading that responsibility to community members seemed 
more realistic to me, like how Vite handles its add-ons and plugins. As for Rust, because it's the best language out there 🦀!
</details>
