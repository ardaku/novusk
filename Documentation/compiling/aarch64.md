# Compiling aarch64 Novusk

When compiling aarch64 Novusk you need to decide what board you want to support. Read ``aarch64/boards.md`` for a list
of supported aarch64 boards.

To compile aarch64 Novusk, this is the main command:
```commandline
make all ARCH=aarch64 TARGET_ARCH=aarch64 DEVICE=<device name>
```

An board example:
```commandline
make all ARCH=aarch64 TARGET_ARCH=aarch64 DEVICE=virt
```

This command will compiling aarch64 Novusk for the Qemu Virt board.