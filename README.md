# SquidClient
> HTB Unbalanced edition

## What is "Unbalanced"?

It's that:

![image](https://user-images.githubusercontent.com/24318966/89657703-24c96200-d8d6-11ea-80f0-41f4c7e624e4.png)

## Usage:

![image](https://user-images.githubusercontent.com/24318966/89705414-e4b1c000-d965-11ea-961b-96f17a2f9390.png)

## Why?

Well, long story short:

![image](https://user-images.githubusercontent.com/24318966/89657996-a6b98b00-d8d6-11ea-817d-2f2deb426400.png)
![Screenshot at Aug 07 18-53-34](https://user-images.githubusercontent.com/24318966/89664167-6579a900-d8df-11ea-8b8e-6d732b767a30.png)

Also, there is **nothing** relevant on GitHub. I, personally, have solved the cachemgr part of this box using raw HTTP requests and a `netcat` utility.

This program does exactly the same thing I did, so you don't have to do it yourselves =)

## Building:

I'll leave the binary for MacOS on the [Releases](https://github.com/limitedeternity/squidclient/releases) page.

Others will need to compile it. Don't worry, it's not that hard:

1. Install the Rust toolchain using [rustup.rs](https://rustup.rs/).

2. `git clone` this repo.

3. Run `cargo build --release` there.

## Notes:

If I've helped you, let me know by starring this repository and/or giving me +rep on [my profile page](https://app.hackthebox.eu/profile/369200).

## Meta:

Distributed under the GPL-3.0 license. See ``LICENSE`` for more information.

[@limitedeternity](https://github.com/limitedeternity)
