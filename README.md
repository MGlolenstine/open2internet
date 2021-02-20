[![Gitpod ready-to-code](https://img.shields.io/badge/Gitpod-ready--to--code-blue?logo=gitpod)](https://gitpod.io/#https://github.com/MGlolenstine/open2internet)

[![Build status](https://ci.appveyor.com/api/projects/status/jnsp3aqljkyuq0y5?svg=true)](https://ci.appveyor.com/project/MGlolenstine/open2internet)

[![Discord](https://img.shields.io/discord/297017452737331200.svg?label=&logo=discord&logoColor=ffffff&color=7389D8&labelColor=6A7EC2)](https://discord.gg/KZM8nf6)

# Open 2 Internet
This is a binary rust package, that allows you to redirect and automatically open ports to your "LAN shared" Minecraft single player world.

It features
- Custom Lease time
- Custom external port
- List of running Minecraft LAN worlds
- GUI frontend

TODO:
- Get more info from Minecraft server (version, world name,...)


# Build it yourself
Clone it
```
$ git clone https://github.com/mglolenstine/open2internet
```
Build it
```
$ cargo build --release
```
you should have your executable located in the `target/release/open2internet`

or run it directly
```
$ cargo run
```

or you could just skip all of that and just type
```
$ cargo install --git https://github.com/mglolenstine/open2internet
```

Current progress can be seen on our [board](https://boards.mglolenstine.xyz/b/YszTuok5GKCKiXoa7/open2internet).

Thanks for checking out the repository!
