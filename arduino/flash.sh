cargo objcopy --release -- -O binary app.bin
~/Library/Arduino15/packages/arduino/tools/bossac/1.9.1-arduino2/bossac -d --port=cu.usbmodem1401 -U -i -e -w app.bin -R