cargo build && ./target/debug/big-num $1 $2
echo "\n"
python3 -c "print($1 / $2)"