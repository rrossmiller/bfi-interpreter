rm bfi
clear
zig build
mv zig-out/bin/bfi .
./bfi ../prg.bf
