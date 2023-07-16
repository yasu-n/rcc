#!/usr/bin/env fish
function assert
    set expected $argv[1]
    set input $argv[2]

    cargo run --quiet -- "$input" > tmp.s
    cc -o tmp tmp.s -Wa,--noexecstack
    ./tmp
    set actual "$status"

    if test "$actual" = "$expected"
        echo "$input => $actual"
    else
        echo "$input => $expected expected, but got $actual"
        exit 1
    end
end

assert 0 0
assert 42 42
assert 21 "5+20-4"
assert 41 " 12 + 34 - 5 "

echo OK
