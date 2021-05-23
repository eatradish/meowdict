# meowdict
moedict.tw web client

## Usage
```
$ ./meowdict 萌
動:
1.發芽。
2.發生。
名:
1.草木初生的芽。
2.事物發生的開端或徵兆。
3.人民。
4.姓。如五代時蜀有萌慮。

$ ./meowdict 什麼
什麼:
1.疑問代名詞，專指事物。如：「你在做什麼？」也作「甚麼」。
2.指示代名詞，泛指一般事物。如：「心裡想什麼，就說什麼，別這樣吞吞吐吐的。」也作「甚麼」。
3.疑問形容詞。如：「你住在什麼地方？」《文明小史．第三四回》：「這是部什麼書，我還不曉得名目，請悔兄指教。」也作「甚麼」。
4.表不定或虛指的形容詞。《紅樓夢．第四八回》：「為這點小事，弄得人坑家敗業，也不算什麼能為。」《文明小史．第三四回》：「這部書沒有什麼道理。」也作「甚麼」 。

$ ./meowdict 空穴來風
空穴來風:
1.有空穴，就有風吹來。語出《文選．宋玉．風賦》：「臣聞於師：『枳句來巢，空穴來風，其所託者然，則風氣殊焉。』」後比喻流言乘隙而入。如：「那些空穴來風的傳聞，不足以採信。」
```

## Installation
```
$ cargo build --release
# install -Dvm755 target/release/meowdict /usr/local/bin/meowdict
```

## Dependencies
Building:
- Rust w/ Cargo
- C compiler
- make (when GCC LTO is used, not needed for Clang)

Runtime:
- Glibc
