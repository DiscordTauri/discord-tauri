<h1 align="center">Discord Tauri</h1>

<p align="center">
    Discord Tauri is a work in progress Discord wrapper made in <a href="https://tauri.studio">Tauri</a>.
</p>

---

# Purpose

I asked myself, how would Discord be if it was built in Tauri? I could only imagine super low memory usage so I got a PoC working in a week.

This project's purpose is to, not only know how Discord would be if it was built in Tauri; but also learn about Discord inner workings and possibly get more people to know about [Tauri](https://tauri.studio)!

# Benchmarks

Made with the same account, same procedure and in the same system.
Your results may vary.
If these results are good, they are thanks to Tauri, not because the Discord dev team is bad.
If these results are bad, well, Tauri is still a good library! Also, good job Discord!

| Detail                         | Discord Tauri | Discord   |
| ------------------------------ | ------------- | --------- |
| Installer Size                 | 2.83 MiB      | 67.57 MiB |
| Installed Size (without Cache) | 20 MiB        | 326 MiB   |
| Memory Usage                   | 254 MB        | 233 MB    |
| Launch Time                    | 5.08s         | 7.41s     |

The following results were obtained with an almost brand new account: 3 servers, 1 friend.

| Detail         | Discord Tauri | Discord   |
| -------------- | ------------- | --------- |
| Memory Usage   | 175 MB        | 178 MB    |
| Launch Time    | 3.06s         | 2.83s     |

---

# List of features
- [x] Wrapping of Discord
- [x] Expandability with Tauri plugins
- [x] Window Bar
- [x] Fast AF Loading
- [x] Desktop Tray
- [ ] Rich Presence Support
- [x] Notifications
- [x] Window Resizing
- [ ] File Dropping
- [ ] Push To Talk
- [ ] Custom Settings
- [ ] Works on Linux

---

# Contributing

## Tauri Dependencies

First, you will need the dependencies needed for Tauri. If you are on Windows, make sure to install the Microsoft Visual Studio C++ Build Tools and WebView2.

You can see the process depending on your operating system <a href="https://tauri.studio/en/docs/getting-started/intro">here</a>! (Linux doesn't work for now!)

## Prerequisites
- [Git](https://git-scm.com)
- [Node.js](https://nodejs.org/en/) with [yarn](https://yarnpkg.com/getting-started)
- Command line of your choice

### Clone the repository
```ps
git clone https://github.com/DiscordTauri/discord-tauri.git
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
### Build a .msi
The .msi file will be in /src-tauri/target/release/bundle/msi
```ps
yarn tauri build
```

## Code style?
Use `cargo fmt` in `/src-tauri` to use our code style (cough cough, not that we use the default Tauri one)

---

# FAQ

### What is Discord Tauri?
It's a Discord wrapper that uses the lightweight library [Tauri](https://tauri.studio). In the current state of development, we are replicating the original Discord client behavior.
