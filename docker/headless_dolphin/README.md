# What is this?

This directory contains configuration lifted from a Dolphin Emulator installation.
The configuration is setup in such a way that:
- Logger output will be streamed to STDERR
- Only OSREPORT_HLE events will be streamed
- MMU support is off (required to run Dolphin inside Docker)
- Quit immediately on shutdown, rather than showing an extra confirm dialog box.

This configuration was created by modifying the Dolphin settings in the GUI,
and afterwards copying the config folder from its location (which varies based on how you installed Dolphin) to inside here.
