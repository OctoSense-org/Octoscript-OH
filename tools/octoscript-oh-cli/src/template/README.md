# My Octoscript App

A native OpenHarmony app: a web frontend, Rust for everything else.

## Build it

The app shell — the ArkTS entry point, the DevEco project, the Rust bridge —
lives in a Octoscript-OH checkout rather than in here. This project is the frontend
and your own native code. Clone one beside this directory, or set `OCTOSCRIPT_OH`:

    git clone https://github.com/OctoSense-org/Octoscript-OH.git ../Octoscript-OH

    npm install
    npm run build
    ./build.sh

## Develop it

    octoscript-oh dev        # opens a USB tunnel to the phone
    npm run dev
    OCTOSCRIPT_DEV_SERVER=http://127.0.0.1:5173 ./build.sh

Frontend edits reload on the device. Rust edits still need a rebuild.

The tunnel is over USB rather than Wi-Fi, so there is no network to configure.

## Add a native capability

Write a tool in `plugin/src/lib.rs` and call it by name from the frontend:

    window.octoscript.invoke('app.greet', { name: 'world' })

Two edits in the Octoscript-OH checkout link it in, because the `.so` is built
there and only the crate that produces it can pull a plugin into the binary:

  * add `my-octoscript-app-plugin = { path = "../my-app/plugin" }` to
    `crates/octoscript-oh-webview/Cargo.toml`
  * add `my_octoscript_app_plugin::register(r);` beside the existing plugin in
    `mount()`, in `crates/octoscript-oh-webview/src/lib.rs`

That is the part of the story still to be automated: today the shell is a
checkout you build against rather than a dependency your project owns.

## Configure it

`octoscript.toml` holds the app's name, bundle id, version, icon, frontend
directory and declared permissions.
