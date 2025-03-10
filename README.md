<!-- markdownlint-disable MD041 -->
[//]: # (auto_md_to_doc_comments segment start A)

# ssh_auth_cargo_publish

[//]: # (auto_cargo_toml_to_md start)

**Store and use encrypted secret_token for crates.io with SSH key**  
***version: 1.0.4 date: 2024-04-30 author: [bestia.dev](https://bestia.dev) repository: [GitHub](https://github.com/automation-tasks-rs/ssh_auth_cargo_publish)***

 ![obsolete](https://img.shields.io/badge/obsolete-red)
 ![rustlang](https://img.shields.io/badge/rustlang-orange)
 ![crates-io](https://img.shields.io/badge/crates_io-orange)

[//]: # (auto_cargo_toml_to_md end)

 [![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/automation-tasks-rs/ssh_auth_cargo_publish/blob/main/LICENSE)
 [![Rust](https://github.com/automation-tasks-rs/ssh_auth_cargo_publish/workflows/rust_fmt_auto_build_test/badge.svg)](https://github.com/automation-tasks-rs/ssh_auth_cargo_publish/)
 [![crates.io](https://img.shields.io/crates/v/ssh_auth_cargo_publish.svg)](https://crates.io/crates/ssh_auth_cargo_publish)
 [![Documentation](https://docs.rs/ssh_auth_cargo_publish/badge.svg)](https://docs.rs/ssh_auth_cargo_publish/)
 [![Lib.rs](https://img.shields.io/badge/Lib.rs-rust-orange.svg)](https://lib.rs/crates/ssh_auth_cargo_publish/)  
 [![Newest docs](https://img.shields.io/badge/newest_docs-blue.svg)](https://automation-tasks-rs.github.io/ssh_auth_cargo_publish/ssh_auth_cargo_publish/index.html)
 ![ssh_auth_cargo_publish](https://bestia.dev/webpage_hit_counter/get_svg_image/648533659.svg)

[//]: # (auto_lines_of_code start)
[![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-266-green.svg)](https://github.com/automation-tasks-rs/ssh_auth_cargo_publish/)
[![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-264-blue.svg)](https://github.com/automation-tasks-rs/ssh_auth_cargo_publish/)
[![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-56-purple.svg)](https://github.com/automation-tasks-rs/ssh_auth_cargo_publish/)
[![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/automation-tasks-rs/ssh_auth_cargo_publish/)
[![Lines in tests](https://img.shields.io/badge/Lines_in_tests-184-orange.svg)](https://github.com/automation-tasks-rs/ssh_auth_cargo_publish/)

[//]: # (auto_lines_of_code end)

Hashtags: #maintained #ready-for-use #rustlang #automation #workflow  
My projects on GitHub are more like a tutorial than a finished product: [bestia-dev tutorials](https://github.com/bestia-dev/tutorials_rust_wasm).  
I recommend using the [CRUSTDE - Containerized Rust Development Environment](https://github.com/CRUSTDE-ContainerizedRustDevEnv/crustde_cnt_img_pod) to write Rust projects on Linux, isolated from your system.  

## OBSOLETE

This will not be a separate crate anymore. This will be a module file that the developer copy-paste into his source code into automation_tasks_rs folder.
I decided that the developer must have complete control over this code in its own project.

## Motivation

To access crates.io with `cargo publish` you need an access secret_token.  
IMPORTANT: Treat access secret_tokens like your password and keep them secret. Store your secret_tokens securely in a credential manager for example.  
Access secret_tokens are impossible to remember for an average human. We need to store them somewhere.  
This command stores the crates.io secret_token:

```bash
cargo login
```

WARNING: Be aware that by default they store the secret_token in "plain-text" in the file: `~/.cargo/credentials`.  
Ok, I see there was some development in this area and now is possible to use "credentials providers".

I want to secure this secret_token with encryption with an SSH key.  
We have already a lot of experience creating, managing and securing our SSH keys. The private key is secured by a passphrase we can remember and type. Every use of the secret_token will need user interaction to type the passphrase. Very secure.  

If we are very self-confident in our current session, we can store the SSH key in ssh-agent and write our passphrase only once.  
WARNING: a dedicated attacker could read from ssh-agent and discover the access secret_token without our user interaction. Use this at your discretion.  

## Replacement command

Put the executable `ssh_auth_cargo_publish` into the folder you intend to use it.  
After copying, make it executable with `chmod +x ssh_auth_cargo_publish`.  
Instead of `cargo publish ...` use `ssh_auth_cargo_publish`.  
If it finds the encrypted secret_token it will ask you for the passphrase to the private SSH key.
Else it will ask you to store the secret_token.

## Development details

Read the development details in a separate md file:
[DEVELOPMENT.md](DEVELOPMENT.md)

## Releases changelog

Read the releases changelog in a separate md file:
[RELEASES.md](RELEASES.md)

## TODO

And code happily ever after...

## Open-source and free as a beer

My open-source projects are free as a beer (MIT license).  
I just love programming.  
But I need also to drink. If you find my projects and tutorials helpful, please buy me a beer by donating to my [PayPal](https://paypal.me/LucianoBestia).  
You know the price of a beer in your local bar ;-)  
So I can drink a free beer for your health :-)  
[Na zdravje!](https://translate.google.com/?hl=en&sl=sl&tl=en&text=Na%20zdravje&op=translate) [Alla salute!](https://dictionary.cambridge.org/dictionary/italian-english/alla-salute) [Prost!](https://dictionary.cambridge.org/dictionary/german-english/prost) [Nazdravlje!](https://matadornetwork.com/nights/how-to-say-cheers-in-50-languages/) 🍻

[//bestia.dev](https://bestia.dev)  
[//github.com/bestia-dev](https://github.com/bestia-dev)  
[//bestiadev.substack.com](https://bestiadev.substack.com)  
[//youtube.com/@bestia-dev-tutorials](https://youtube.com/@bestia-dev-tutorials)  

[//]: # (auto_md_to_doc_comments segment end A)
