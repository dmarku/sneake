# Installing Sneake From Source

1. Install Rust and Cargo on your system: <https://www.rust-lang.org/learn/get-started>

1. Clone the repository and change into its directory:

   ```
   git clone git@github.com:dmarku/sneake.git
   cd sneake
   ```

1. Build and run the main executable:

   ```
   cargo run --release
   ```

   **Note:** this will download _a lot_ of dependencies and compile them. This may take a while on the first run. On an Intel i5-7200U, that's about 5 minutes and 700 MB of disk space.

1. Play!

   - move around with the arrow keys
   - quit the game with Escape
   - reset the level with "R" if you're stuck, you've finished the level or whenever you feel like it
