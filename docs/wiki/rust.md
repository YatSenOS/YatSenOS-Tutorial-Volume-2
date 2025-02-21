# Rust 语言基础

## 语言学习

Rust 是一门系统编程语言，它有更强的类型检查和内存安全保证，可以避免很多 C/C++ 中常见的内存错误，如缓冲区溢出、空指针引用等。

Rust 语言的基础语法可以参考 [Rust圣经](https://course.rs/) 或者 [Rust Programming Language](https://doc.rust-lang.org/book/) 等资料。

当熟悉 Rust 的语法与特性后，可以尝试去完成 [Rustlings](https://github.com/rust-lang/rustlings) 的练习，这些练习可以帮助你更好的理解 Rust 语言的特性。

本实验中使用的语言版本为 2024 版本，发布于 2025 年 2 月 20 日。

其他可参考的学习资料：

- [Rust 之旅（交互式课程）](https://tourofrust.com/00_zh-cn.html)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rust Cookbook](https://rust-lang-nursery.github.io/rust-cookbook/)
- [清华的 Rust 课程](https://lab.cs.tsinghua.edu.cn/rust/)

值得注意的是，本实验内容并不要求你对 Rust 语言有深入的了解，只需要你能够 **理解并使用** Rust 语言的以下内容：

- **基本语法**

    变量绑定、常量、表达式、基本类型、条件语句、模式匹配、函数

- **所有权与结构化数据**

    所有权、移动语义、借用与可变引用、结构体、元组结构体、单位元结构体、枚举

- **标准库**

    `String`、`Vec<T>`、`Result<T, E>`、`Option<T>`、错误处理、单元测试

- **泛型、特型与生命周期**

    泛型、特型、标准库提供的常用特性、生命周期入门

- **项目管理与常用库**

    Cargo 项目结构、命名规范、智能指针

## 善用 `docs.rs`

Rust 提供了 [docs.rs](https://docs.rs/) 来帮助查看 crate 的文档，你可以在其中搜索你需要的 crate，然后查看其文档。

在 Rust 中，你可以通过特殊的语法，借助 Markdown 的语法来编写文档，这些内容被称作文档注释，你可以在 [注释和文档](https://course.rs/basic/comment.html) 部分了解到这些内容。

这些文档注释会被编译器提取出来，然后生成文档，并转换为 HTML 以供查看。

你可以通过上述文档进一步详细了解这些内容。

由于 Rust 通过源码进行依赖分发，所以对于你不了解的实现、函数内容，**可以通过编辑器的跳转能力，直接查看源码及其文档的内容**。
