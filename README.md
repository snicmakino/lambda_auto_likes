# lambda_auto_likes
This application likes Twitter posts with certain keywords automatically using AWS lambda  
AWS lambdaを利用したTwitter自動いいねツール  

## Doesn't work
クロスコンパイルしたバイナリが実行できないため、Lambda上でもうまく動作していないようです  
```bash
$ cargo run --target=x86_64-unknown-linux-musl --release
    Finished release [optimized] target(s) in 0.0 secs
     Running `target/x86_64-unknown-linux-musl/release/lambda_auto_likes`
error: could not execute process `target/x86_64-unknown-linux-musl/release/lambda_auto_likes` (never executed)
```
現状、解決は出来ていません
