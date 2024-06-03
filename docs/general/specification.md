---
no_comments: true
---

# 代码与提交规范

!!! tip "部分内容引用修改自 [代码规范](https://lab.cs.tsinghua.edu.cn/rust/projects/coding_convention/)"

本页面列举了一些常见的代码规范要求。部分要求并不强制，但是建议尽量遵守和学习。

## 代码风格

=== "简洁"

    保证代码的简洁：

    - 有整齐的缩进，建议用空格缩进而非 tab，两个空格或者四个空格作为一级缩进都可以；
    - 每一行不要有太多字符，例如不超过 80 - 100 个字符；

=== "注释"

    在代码中编写适当的注释：

    - 在比较复杂的代码块前，描述代码的功能；
    - 过于简单的代码，一般不写注释；
    - 函数一般要编写注释，包括其功能，参数和输出；
    - 建议用英文，中文也可以，但是注意要用 UTF-8 编码。
    - 遵循 [Rustdoc 的约定](https://doc.rust-lang.org/rustdoc/how-to-write-documentation.html)

=== "空白"

    代码中应该有适当的空格和空行：

    - 函数中，实现不同功能的代码间需要添加空行；
    - 操作符前后应该有空格，例如 `c = a + b`；
    - 保持前后一致的风格。

=== "命名"

    变量的命名，应该符合一定的规范：

    - 符合 Rust 编译器建议的变量命名规范；
    - 尽量用英文单词，而不是中文拼音首字母；
    - 命名与上下文要相关；
    - 不用类似 `a, b, c, d` 的命名方式。

建议使用工具来辅助代码风格的检查：

-   使用 `rustfmt`，`cargo fmt --all` 命令来格式化代码；
-   使用 `clippy`，`cargo clippy` 来检查代码风格。
-   使用 [typos](https://github.com/crate-ci/typos) 检查拼写错误，可以使用 `cargo install typos-cli` 安装。

!!! note "请注意，由于项目 target 不尽相同，`clippy` 需要在每一个 `package` 下使用。"

## Git 相关

### 提交历史

-   每个提交都应该有一定的意义，例如实现了新功能，修复了一个问题，定义了新的函数；
-   比较复杂的程序，要边开发边提交，而不是写完了再一次性提交；
-   不强求线性历史，**但是不允许使用 force push**。

### 提交消息

-   简单明了地描述这个提交的内容；
-   建议用英文写，用中文写也可以；
-   不要编写的过于详细或过于简略；
-   可以采用一些格式，例如 [**Conventional Commits**](https://www.conventionalcommits.org/en/v1.0.0/#examples)；
-   不掺杂个人情绪；
-   可以添加一些 Emoji，[gitmoji](https://gitmoji.dev/) 为提交说明中使用的 Emoji 提出了一些建议，可以参考。

### 代码打包

在进行代码提交时，使用如下命令将代码打包：

```bash
git archive --format zip -o ../lab1.zip v0.1
```

`git` 会遵守 `.gitignore` 文件中的规则，不会将不必要的文件打包。从而无需手动删除不必要的文件，或者频繁使用 `cargo clean` 等命令。

**在期末最后的实验报告提交时，需要提交整个代码仓库**，你可以使用如下命令进行仓库的打包，将 `00000000` 位置替换为你的学号：

```bash
git bundle create ../ysos-00000000.bundle --all
```

??? note "给 TA 的注释"

    可以直接使用 `git clone` 查看 `bundle` 文件：

    ```bash
    git clone ysos-00000000.bundle /path/to/repo
    ```

    之后可以直接在 `/path/to/repo` 目录下查看提交记录、开发时间等内容。
