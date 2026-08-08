

+-------------------------------------------------------------+
|                       SANDBOX CRATE                         |
|  - Only imports `forge`                                     |
|  - Defines game logic, layers, and custom graphics          |
|  - Knows NOTHING about SDL3                                 |
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
