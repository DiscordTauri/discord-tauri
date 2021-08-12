<h1 align="center">Discord Tauri</h1>

<p align="center">
    Discord Tauri is a work in progress Discord wrapper made in <a href="https://tauri.studio">Tauri</a>.
</p>

---

# Purpose

I asked myself, how would Discord be if it was built in Tauri? I could only imagine super low memory usage so I got a PoC working in a week.

This project's purpose is to, not only know how Discord would be if it was built in Tauri; but also learn about Discord inner workings and possibly get more people to know about [Tauri](https://tauri.studio)!

# Benchmarks

Made with same procedure and in the same system (WINDOWS).

Your results may vary.

If these results are good, they are thanks to Tauri, not because the Discord dev team is bad.

If these results are bad, Tauri is still a great library! Also, good job Discord!

| Detail                         | Discord Tauri | Discord   |
| ------------------------------ | ------------- | --------- |
| Installer Size                 | 2.83 MiB      | 67.57 MiB |
| Installed Size (without Cache) | 20 MiB        | 326 MiB   |
| Max. Memory Usage              | 650 MB        | 630 MB    |
| Avg. Memory Usage (after load) | 550 MB        | 535 MB    |
| Min. Memory Usage (after load) | 500 MB        | 490 MB    |
| Launch Time                    | 5.08s         | 7.41s     |

The following results were obtained with an almost brand new account: 3 servers, 1 friend.

| Detail                         | Discord Tauri | Discord   |
| ------------------------------ | ------------- | --------- |
| Max. Memory Usage              | 580 MB        | 640 MB    |
| Avg. Memory Usage (after load) | 512 MB        | 516 MB    |
| Min. Memory Usage (after load) | 460 MB        | 450 MB    |
| Launch Time                    | 3.06s         | 2.83s     |

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

You can see the process depending on your operating system [here](https://tauri.studio/en/docs/getting-started/intro)! (Linux doesn't work for now!)

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

### Is this against Discord's ToS?
Yes. Every Discord mod is against the ToS, altough, it seems like Discord still doesn't care too much about it. Quoting [Wired's article about BetterDiscord](https://www.wired.com/story/betterdiscord-lets-users-mod-chat-app/): "BetterDiscord is not an officially sanctioned app; and likely it breaks Discord’s terms of service, which prohibit modifying Discord. But the software has been installed more than 5.3 million times since 2015, and its developers say they have not seen Discord take action against users for modifying the client".

### Is this project abandoned?
No. But I'm a student and it's hard to add features and fix bugs while having to do homework. Hopefully I will be able to continue discord-tauri in the winter :)