# **Rusty Palette** ✨

A Rust tool for applying a set of themes and wallpapers to your system and apps via a configuration file.

## **Overview** 🌟

**Rusty Palette** applies visual customizations from JSON configs, including:

1. Setting wallpapers.
2. Changing system app themes.
3. Customizing Papirus Folder colors.

---

## **Pre-Requisites** 🔧

1. **Rust**: Install `rustup` to compile and run. [Install Rust](https://www.rust-lang.org/tools/install).

2. **Papirus Folders**: Install `papirus-folders` for folder icon customization. [Install Papirus Folders](https://github.com/PapirusDevelopmentTeam/papirus-folders)

3. **Feh**: Install `feh` for setting wallpapers.

This command should install all 3 packages above:

   ```bash 
   sudo pacman -S rustup feh papirus-folders && rustup default stable
   ```

---

## **How to Use** 🚀

1. **Clone and Build**:

   ```bash
   git clone https://github.com/O-ELMA/rusty_palette.git && cd rusty_palette && cargo build --release
   ```

2. **Run the Program**:

   ```bash
   cargo run
   ```

   Once you press enter, this will apply the selected theme, wallpapers, and icon color changes.

---

## **Configuration** ⚙️

The tool uses two configuration files:

1. **`apps.json`**: Specifies apps to customize.
2. **`themes.json`**: Contains theme names with wallpapers.
