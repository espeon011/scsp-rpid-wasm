# scsp-rpid-wasm

[DIDP](https://github.com/domain-independent-dp/didp-rs) の Rust インターフェースである [RPID](https://github.com/domain-independent-dp/rpid) を用いて定義した最適化モデルを WASM ターゲットでコンパイルし, 
適当な `index.html` を添えることで簡素な UI を持った Web アプリのようにしてみた. 

問題は最短共通超配列問題 (SCSP) を扱う.
定式化は[動的計画法ベースの数理最適化ソルバDIDPPyで最短共通超配列問題を解く](https://zenn.dev/okaduki/articles/7f9a3f3c54bc98)を参考. 
あえて記事とは異なる dual bound を採用しているため, 巨大な問題では事前計算に時間がかかりすぎることがある. 
制限時間は 10 秒で, スレッド数は 1.

## Build

```bash
wasm-pack build --target web
```
