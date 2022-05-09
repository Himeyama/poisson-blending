# Poisson blending
Poisson Image Editing によってシームレスな画像合成を行います。

## 実行環境
- Rust をインストール済み
- Cargo を実行可能

## インストール
以下のようなコマンドを実行していくと、`poisson-image-editing` コマンドがインストールされます。

```bash
git clone --depth=1 https://github.com/himeyama/poisson-blending ~/poisson-blending
cd ~/poisson-blending/
cargo install --path .
```

## アンインストール
```bash
cargo uninstall poisson-image-editing
```

## デモンストレーション
```bash
cd ~/poisson-blending/docs/images
rm -f output.png
poisson-image-editing
```

## 仕様
ターゲット画像 `target.png` と ソース画像 `source.png` から `output.png` を合成します。

**ターゲット画像及びソース画像のサイズ(高さ及び幅)は同じである必要があります。**

- target.png  
  ![docs/images/target.png](docs/images/target.png)
- source.png  
  ![docs/images/source.png](docs/images/source.png)
- output.png  
  ![docs/images/output.png](docs/images/output.png)

### オプション
短いオプション|長いオプション|説明|
|:--:|:--:|:--:|
|-t TARGET|--target TARGET|ターゲット画像を指定します。デフォルトは `target.png` です。|
|-s SOURCE|--srouce SOURCE|ソース画像を指定します。デフォルトは `source.png` です。|
|-o FILENAME|--output FILENAME|出力画像を指定します。デフォルトは `output.png` です。|
|-r NUMBER|--repeat NUMBER|繰り返し回数を指定します。デフォルトは `10` です。|
||--verbose|詳細を表示します。|
|-v|--version|バージョンを表示します。|
|-h|--help|使用法を表示します。|

## 参考
- http://opencv.jp/opencv2-x-samples/poisson-blending/
- https://www.jstage.jst.go.jp/article/itej/64/5/64_5_729/_pdf