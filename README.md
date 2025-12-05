pixi-assistant
===============

Helper CLI to verify that the pixi cache directory has enough free space before running pixi operations.

Usage
-----

```sh
pixi-assistant check --gb <required_space_in_gb>
```

- The command reads the pixi cache location via `pixi info --json` and checks available space on that filesystem.
- Exits with code `0` when the requirement is met, otherwise exits with `1` and prints further information.

Notes
-----

- Requires pixi to be installed and discoverable on `PATH`.
- This tool can be used in a pixi task to ensure dependent tasks can successfully build their environment.
