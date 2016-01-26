# galvanizer

Continuous integration server written in Rust


## Prerequisites

galvanizer is built on top of several technologies which should be installed before continuing
with installation:

* [Rust programming language](http://rust-lang.org)
* [Node.js and npm](https://nodejs.org)
* [Bower package manager](http://bower.io/) (`npm install -g bower`)
* [Gulp task runner](http://gulpjs.com) (`npm install -g gulp`)


## Installation

1. Clone repository: `git clone https://github.com/jaxx/galvanizer.git`
2. `cd galvanizer; cargo build --release`. The binary will now be in `./target/release/galvanizer`
3. Open node.js console and install project dependencies: `npm install`
4. Use gulp default task to compile public assets for website: `gulp`
5. Start application: `cargo run`
6. In your browser navigate to [http://localhost:3000](http://localhost:3000)
