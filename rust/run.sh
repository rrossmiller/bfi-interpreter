rm bfi
clear
cargo build
mv target/debug/bfi .
echo "-----"
./bfi ../prg.bf 
# ./bfi ../prg.bfa

