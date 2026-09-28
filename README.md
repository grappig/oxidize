# Oxidize

Oxidize is an experimental container runtime written as a personal learning project.

I am building it to see what happens behind the scenes when an application runs in a container. The project will start small and grow as I learn more.

The main focus is:

- running a program in an isolated environment;
- keeping the program separate from the host system;
- learning how resource limits affect a running process.

## Current usage

```text
oxidize init <rootfs-path>
oxidize inspect <rootfs-path>
oxidize run [--rootfs <rootfs-path>] [--isolate] <program> [arguments...]
```

The `init` command creates the basic directories commonly found in a linux
root filesystem. The `inspect` command checks whether those directories are
present and reports anything missing. The regular `run` command starts a
program on the host. On Unix, the `--rootfs` form starts it with that
directory as its filesystem root. The rootfs must already contain the
program and any files it needs.

On linux, `--isolate` runs the program in its own PID, mount, UTS and IPC
namespaces. Inside, the program is PID 1, and changing the hostname or
mounting something does not affect the host. This needs root. The 2 options
can be used together:
```text
sudo oxidize run --isolate sh -c 'echo $$'
sudo oxidize run --rootfs <rootfs-path> --isolate /bin/sh
```
The first command prints `1`. `/proc` is not mounted again yet, so `ps` still shows
the host's processes.

## What the code does not do yet

Although the project is learning how containers work, the current code is
still only the beginning. It does not yet:

- create network, user or cgroup namespaces
- limit memory or CPU
- mount `/proc`, `/sys`, or `/dev`
- set up networking
- change users or drop privileges
- copy programs and their dependencies into a rootfs
- prevent all host access
- run rootfs isolation on Windows
- provide a GUI

The current rootfs feature is therefore experimentation, not
a complete secure container runtime.
