# Copper Mod Manager

A high-performance, zero Config-As-Code, Minecraft Mod Manager CLI tool, to manage mods from Modrinth for Fabric.

```sh
Tool to manage Minecraft mods for Fabric + Modrinth

Usage: cmm.exe <COMMAND>

Commands:
  add      Add a mod
  rm       Remove a mod
  list     Show mods in mods directory
  latest   Check if mods have a new version, and option to download them
  support  Check if mods are supported for the next or older Minecraft version, and option to download them
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

Let's break the sentence down:

> **high-performance**

Made with Rust, applied concurrency

> **zero Config-As-Code**

This tool does **NOT** create any config files. Making the root of your Minecraft directory readable, and maintainable.

> **CLI tool**

Short for Command Line Interface, this tool is made for the terminal.

> **manage mods for Fabric from Modrinth**

This tool only manages mods that come from Modrinth and is only compatible for the Fabric Mod Loader.

---

### Installation & usage

1. On Linux, and Windows you can install copper-mod-manager via the [releases](https://github.com/prisecano/copper-mod-manager/releases) page.
2. Put `cmm` or `cmm.exe` in the root directory of your minecraft instance/server.
3. Give permission to the file `cmm` or `cmm.exe` to be able to execute it, by typing:

**Linux**
```sh
chmod +x cmm
```

4. Done! To run the tool, type:

**Linux**
```sh
./cmm
```

**Windows**
```sh
.\cmm
```

### Contributing

#### Branch strategy

1. Fork this project.
2. Create relevant branch:
For fixes: `dev/fix-<YOUR-FIX-BRANCH_NAME>`
For features: `dev/feature-<YOUR-FEATURE-BRANCH_NAME>`
3. After you finished development, create a PR (Pull Request) to this dev branch.
4. Discuss and push changes if needed.
5. Lead developer happy? Lead developer will merge your PR into dev.

**WIP**
