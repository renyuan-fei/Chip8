fn main() {
    // Specific the SDL2 in system
    println!("cargo:rustc-link-search=native=/opt/homebrew/opt/sdl2/lib");
}