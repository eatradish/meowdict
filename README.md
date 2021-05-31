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

### Simplified Chinese <-> Traditional Chinese
meowdict currently supports converting the input Simplified Chinese to Traditional Chinese for searching, to avoid the problem that the search does not include Simplified Chinese words in the MoeDict.

e.g:
```
$ ./meowdict 老师
Error: Could not find keyword: 老师
$ ./meowdict 老师 --input-s2t
英語：teacher
拼音：lǎo shī
注音：ㄌㄠˇ　ㄕ
1.對傳授道業、學問或技藝者的尊稱。
2.學生對先生的尊稱。
3.科舉時代門生對座主的稱呼。
```

Similarly, meowdict also supports converting the search results into simplified Chinese:

```
$ ./meowdict 老師
英語：teacher
拼音：lǎo shī
注音：ㄌㄠˇ　ㄕ
1.對傳授道業、學問或技藝者的尊稱。
2.學生對先生的尊稱。
3.科舉時代門生對座主的稱呼。
$ ./meowdict 老師 --result-t2s
英语：teacher
拼音：lǎo shī
注音：ㄌㄠˇ　ㄕ
1.对传授道业、学问或技艺者的尊称。
2.学生对先生的尊称。
3.科举时代门生对座主的称呼。
```

Can be used simultaneously:

```
$ ./meowdict 老师 --input-s2t --result-t2s
英语：teacher
拼音：lǎo shī
注音：ㄌㄠˇ　ㄕ
1.对传授道业、学问或技艺者的尊称。
2.学生对先生的尊称。
3.科举时代门生对座主的称呼。
```

If you are using console mode, you can also pass in parameters like this during startup:

```
$ ./meowdict --input-s2t-mode
meowdict > 老师
英語：teacher
拼音：lǎo shī
注音：ㄌㄠˇ　ㄕ
1.對傳授道業、學問或技藝者的尊稱。
2.學生對先生的尊稱。
3.科舉時代門生對座主的稱呼。
```

You can use `--help` to see the modes that can be set:

```
saki@Mag230 [ debug@master ] $ ./meowdict --help
meowdict 0.5.3
Mag Mell
Check chinese keyword from moedict.tw

USAGE:
    meowdict [FLAGS] [INPUT]...

FLAGS:
    -h, --help               Prints help information
    -i, --input-s2t          Convert input to traditional Chinese and search
        --input-s2t-mode     Open console with input-s2t mode
    -r, --result-t2s         Convert result to Simplified Chinese to display
        --result-t2s-mode    Open console with result-t2s mode
    -t, --translation        Get all translation
    -V, --version            Prints version information

ARGS:
    <INPUT>...    Input the keyword to use
```

Similarly, it is possible to set the console:

```saki@Mag230 [ debug@master ] $ ./meowdict 
meowdict > 老师
Could not find keyword: 老师
meowdict > --set-mode-input-s2t
Setting input mode as s2t...
meowdict > 老师
英語：teacher
拼音：lǎo shī
注音：ㄌㄠˇ　ㄕ
1.對傳授道業、學問或技藝者的尊稱。
2.學生對先生的尊稱。
3.科舉時代門生對座主的稱呼。
meowdict > --set-mode-result-t2s
Setting result mode as t2s...
meowdict > 老师
英语：teacher
拼音：lǎo shī
注音：ㄌㄠˇ　ㄕ
1.对传授道业、学问或技艺者的尊称。
2.学生对先生的尊称。
3.科举时代门生对座主的称呼。
meowdict > --unset-mode-all
Unsetting input mode...
Unsetting result mode...
meowdict > 老师
Could not find keyword: 老师
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
