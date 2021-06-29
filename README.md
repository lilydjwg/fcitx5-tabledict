## 一个操作 fcitx5 码表的工具

这是一个继承 [fcitx-mb](https://www.jiemian.com/article/6290864.html) 的工具，但是用于 fcitx5。

并不是很满意，因为 fcitx5 的库缺乏一些特性。

* 只能反查在主码表中的字词的编码（不支持用户添加和删除的字词），也没有办法反查字词的所有编码
* `index` 的含义未知
* 并不能修改主码表，只能修改用户码表
* 无法调整字词出现的顺序

### 使用方法

你需要安装有 Rust cargo 工具。

修改 `src/main.rs` 里的文件路径，然后：

```
cargo run --release
```
