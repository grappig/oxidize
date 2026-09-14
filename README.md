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
oxidize run <program> [arguments...]
```

The `init` command creates the basic directories commonly found in a linux
root filesystem. The `inspect` command checks whether those directories are
present and reports anything missing. The project does not copy programs or
provide isolation yet those are later steps.