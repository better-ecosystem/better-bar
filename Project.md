## Project Structure

### The files structure might be messy and i will fix it later  
.
├── Cargo.toml
├── PATHS.md
└── src
    ├── config                      ---> Config and css parser
    │   └── mod.rs
    ├── main.rs                     ---> Main file
    ├── system
    │   ├── mod.rs
    │   ├── system_info_modules.rs  ---> Right side modules
    │   └── updater.rs              ---> Updates info periodically
    └── ui
        ├── bar.rs                  ---> Main bar file for the panel
        ├── mod.rs
        ├── modules                 ---> Different modules directory
        ├── style                   ---> Default css files
        └── styles.rs               ---> default css loader
