# meowdict
CLI Web client for moedict.tw

## Usage

### Console Mode
```
$ meowdict
meowdict > 我
英語：I
拼音：（語音）wǒ
注音：（語音）ㄨㄛˇ
代：
1.自稱。
《易經．中孚卦．九二》：「我有好爵，吾與爾靡之。」
《詩經．小雅．采薇》：「昔我往矣，楊柳依依；今我來思，雨雪霏霏。」
2.自稱己方。
《左傳．莊公十年》：「春，齊師伐我。」
《漢書．卷五四．李廣傳》：「我軍雖煩擾，虜亦不得犯我。」
形：
1.表示親切之意的語詞。
《論語．述而》：「述而不作，信而好古，竊比於我老彭。」
漢．曹操〈步出夏門行〉：「經過至我碣石，心惆悵我東海。」
名：
1.私心、私意。
《論語．子罕》：「毋意，毋必，毋固，毋我。」
如：「大公無我」。
2.姓。如戰國時有我子。
拼音：（讀音）ě
注音：（讀音）ㄜˇ
1.(一)之讀音。
meowdict > 我 --translation
Deutsch:
ich (mir, mich) <Personalpronomen 1. Pers.&gt (Pron)
English:
I
me
my
francais:
je
moi
```

### (non) Console Mode
```
$ meowdict 萌
拼音：méng
注音：ㄇㄥˊ
動：
1.發芽。
《楚辭．王逸．九思．傷時》：「明風習習兮龢暖，百草萌兮華榮。」
如：「萌芽」。
2.發生。
《管子．牧民》：「惟有道者，能備患於未形也，故禍不萌 。」
《三國演義．第一回》：「若萌異心，必獲惡報。」
如：「故態復萌」。
名：
1.草木初生的芽。
《說文解字．艸部》：「萌，艸芽也。」
唐．韓愈、劉師服、侯喜、軒轅彌明 〈石鼎聯句〉：「秋瓜未落蒂，凍芋強抽萌。」
2.事物發生的開端或徵兆。
《韓非子．說林上》：「聖人見微以知萌，見端以知末。」
漢．蔡邕〈對詔問灾異八事〉：「以杜漸防萌，則其救也。」
3.人民。
如：「萌黎」、「萌隸」。
通「氓」。
4.姓。如五代時蜀有萌慮。

$ meowdict 萌 --translation
Deutsch:
Leute, Menschen  (S)
Meng  (Eig, Fam)
keimen, sprießen, knospen, ausschlagen 
English:
to sprout
to bud
to have a strong affection for (slang)
adorable (loanword from Japanese `萌~え moe, slang describing affection for a cute character)
francais:
germer
bourgeonner
mignon
adorable
```


## Installation
```
$ cargo build --release
# install -vm755 target/release/meowdict /usr/local/bin/meowdict
```

## Dependencies
Building:
- Rust w/ Cargo
- C compiler
- make (when GCC LTO is used, not needed for Clang)

Runtime:
- Glibc
- OpenSSL
- OpenCC (>= 1.1.0)
