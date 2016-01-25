# galvanizer
Continuous integration server written in Rust

## Installation
1. Clone repository: ```git clone https://github.com/jaxx/galvanizer.git```
2. ```cd galvanizer; cargo build --release```. The binary will now be in ```./target/release/galvanizer```
3. Install [node.js](https://nodejs.org/)
4. Open node.js console and install components needed by galvanizer: grunt and bower
  * To install grunt use ```npm install -g grunt-cli```
  * To install bower use ```npm install -g bower```
5. Run ```bower install``` in galvanizer root directory from node.js console
6. Start binary and open browser ```http://localhost:3000```