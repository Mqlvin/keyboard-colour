# Winbond Keyboard Colour
A Linux app to change lighting on your Winbond based keyboards (ZET Gaming, FL Esports etc).

## Installation 🛠️
To install, you can try the precompiled binary in the [releases](aaaaaaaa) page. Simply download and run!<br><br>

Alternatively, you can compile the code from source.<br>
Simply clone the repo, and run `cargo build --release`.<br>
The `target/release` folder will contain the binary `kb-colour`.<br><br>

I'd recommend dropping the binary in your `/usr/bin/` folder so you can access it anywhere - rename it to whatever you'd like.

If this project receives any traction, I may maintain some package repositories for various Linux flavours.

## How to Use 🚀
To use the software, execute the `./kb-colour` binary. A window will pop up with various options to control the lighting effects.<br>
Select a lighting style to your choosing, and click "Apply" to active the new lighting effects.<br><br>
![An image of the software](https://i.imgur.com/JZyCZxB.png)

## Reverse Engineering 👨‍💻
I used [USBPcap](https://desowin.org/usbpcap/) and [Wireshark](https://www.wireshark.org/) on a Windows VM to sniff outgoing packets from the original keyboard software. Changing a single keyboard lighting setting at a time allowed me to map out the instructive bytes of the keyboard, and hence replicate them.<br>I did this because the original software does not have native Linux or MacOS support. <br>

## Licensing 📄
This project is licensed under the [MIT License](https://choosealicense.com/licenses/mit/)<br>
