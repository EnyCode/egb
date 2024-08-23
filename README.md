<div align="center">
  <img src="https://cloud-6u3qr1fur-hack-club-bot.vercel.app/0image.png" />
  <h1>EGB</h1>
  A WIP retro-game emulator for the Sprig. 
</div>
<br />

## Features
- [x] Basic UI
  - [x] Brightness Control
  - [ ] Game selection
- [ ] NES emulator
  - [x] CPU
  - [ ] GPU
  - [ ] Works on Sprig (only on pc right now)
- [ ] GameBoy emulator

## Building
EGB is built using [Rust](https://rust-lang.org), and is therefore a requirement for building and running. 

### Running on the Sprig
Simply run the command below while having your Sprig plugged in on USB Boot mode, and EGB will boot up. 
```
cargo run --target thumbv6m-none-eabi
```

### Emulating locally
This is configured by default, and will open the emulator in a new window. 
```
cargo run
```

## Fun Facts
- I spent far too long trying to make the transition work properly, and with trial and error it does actually work! Just not on PC for some reason.
- This is my first ever emulator! Sadly it doesn't work on the Sprig and without a debug probe it's a bit of a pain to debug.
- Thanks to [this guide](https://bugzmanov.github.io/nes_ebook/index.html) on emulating the NES, I got snake running (though just on the CPU)

![](https://cloud-6u3qr1fur-hack-club-bot.vercel.app/2image.png)
- Also, this is my first ever embedded app ever - in Rust or otherwise! I learnt a lot from it and, while I didn't finish it, I will definitely come back to embedded applications. 

## License
EGB is licensed under Mozilla Public License 2.0 unless otherwise stated. 
