<h1 align="center">Discord-Tauri</h1>

<p align="center">
    Discord-Tauri is a work in progress lightweight wrapper for Discord.
</p>

---

# Contributing

## Tauri Dependencies

First, you will need the dependencies needed for Tauri. If you are on Windows, make sure to install the Microsoft Visual Studio C++ Build Tools and WebView2.

You can see the process depending on your operating system <a href="https://tauri.studio/en/docs/getting-started/intro">here</a>!

## Prerequisites
- [Git](https://git-scm.com)
- [Node.js](https://nodejs.org/en/) with [yarn](https://yarnpkg.com/getting-started)
- Command line of your choice

### Clone the repository
```ps
git clone https://github.com/elJoa/discord-tauri.git
```
### Install the project dependencies
```ps
yarn install
```
At this point, you can edit the code!
### Run discord-tauri
```ps
yarn tauri dev
```
### Build a .exe
The .exe file will be in /src-tauri/target/release
```ps
yarn tauri build
```

## Code style?
Not yet; you can follow the existing code, though.

---

# FAQ

### What is discord-tauri?
It's a Discord wrapper that uses the lightweight library [Tauri](https://tauri.studio). Hopefully, it will allow developers to create plugins or themes. In the current state of development, though, we are still replicating the original Discord client behavior.

### So it will be like BetterDiscord?
Yes! In fact, this project was highly inspired by BetterDiscord.
