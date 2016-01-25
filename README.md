# galvanizer

Continuous integration server written in Rust


## Prerequisites

galvanizer is built on top of several technologies which should be installed before continuing
with installation:

* [Rust programming language](http://rust-lang.org)
* [Node.js and npm](https://nodejs.org)


## Installation

1. Clone repository: ```git clone https://github.com/jaxx/galvanizer.git```
2. ```cd galvanizer; cargo build --release```. The binary will now be in ```./target/release/galvanizer```
4. Open node.js console and install project dependencies: ```npm install```
5. Install third party javascript libraries: ```bower install```
6. Use grunt tasks to compile stylesheets and javascript dependencies:
    * Compile CSS files: ```grunt less```
    * Compile javascript files: ```grunt concat; grunt uglify```
