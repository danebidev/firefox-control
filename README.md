# Firefox control

A simple browser extension to control Firefox with an unix socket.
It is not stable nor secure, use at your own risk.
It doesn't have many features, but they can be _easily_ added.
The extension uses native messaging to communicate with a daemon that manages the unix socket. I've also included a simple cli client to interact with the daemon.

## Features

- Open a new tab
- Close a tab
- Get a list of all tabs
- Change currently active tab
