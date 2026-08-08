# Forge

A Rust game framework built wrapping SDL3.

## Architecture

```
+-------------------------------------------------------------+
|                       SANDBOX CRATE                         |
|  - Only imports `forge`                                     |
|  - Defines game logic, layers, and custom graphics          |
|  - Don't know what backend is using/ in this case SDL3      |
+-------------------------------------------------------------+
                               |
                   Talks only via Forge API
                               v
+-------------------------------------------------------------+
|                        FORGE CRATE                          |
|  - Imports `sdl3` privately                                 |
|  - Bootstraps the OS window and system event loops          |
|  - Converts raw OS/SDL events into custom Forge enums        |
+-------------------------------------------------------------+
```

## Workspace

```
forge/
├── forge/      # core library i want to make this .so in linux 
└── sandbox/    # this only sees the forge apis
```

## Run

```sh
cargo run -p sandbox
```
