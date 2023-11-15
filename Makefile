# NOTE: the linker script has this value hardcoded.
#
# NOTE: for .dsk files, other values (e.g. 0x6000) don't work: ProDOS copies
# the program into memory at offset 0x2000 anyways. (Perhaps because it's a
# "system" program, whatever that means.)
start_addr := 2000

default: dsk

build: clean
	cargo rustc --release -- --emit=llvm-ir
	mkdir -p out
	mos-clang target/release/deps/*.ll panic.ll -o out/main --config mos-common.cfg

wav: build
	c2t -bc8 out/main,$(start_addr) out/main.wav

dsk: build
	cp "disk-images/ProDOS 8.dsk" out/main.dsk
	for f in launcher sysutil fastcopy basic; do ac -d out/main.dsk $$f.system; done
	ac -p out/main.dsk main.system sys 0x$(start_addr) < out/main

clean:
	cargo clean
	rm -rf out/
