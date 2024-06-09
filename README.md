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

## Installation

- Install the extension in the ext directory
- Build the daemon with cargo (or use a release from the releases page)
- Copy the daemon to some folder
- Copy the native app manifest (firefox_control.json) to the Firefox native messaging folder (usually ~/.mozilla/native-messaging-hosts/)
- Change the path in the native app manifest to the daemon
- The extension should start the daemon automatically when it is loaded
