# 使用 typst 编写实验报告

[typst](https://github.com/typst/typst) 是一个新型的基于标记的轻量化排版系统，相较于安装复杂且体积庞大的 LaTex，typst 所需要的空间仅仅约 30M，安装方法也十分简单，下载对应的二进制文件，添加到系统路径即可。对于IDE，推荐使用 VSCode，并安装 Typst LSP 插件实现实时预览。

关于 typst 的一些细节可以参考 [typst doc](https://typst.app/docs/)，同时，推荐使用模板来撰写格式更加更为统一的实验报告，可以参考[模板](https://github.com/GZTimeWalker/GZ-Typst-Templates)，后文给出了一个可能的使用方法。

## 使用示例

!!! note "假设你已经成功安装 typst，并已经加入到系统路径"

### 从文件管理开始

为了方便，我们从头开始，新建一个文件夹，专门用来进行实验报告的撰写。

假设你本地已经克隆了一份[模板](https://github.com/GZTimeWalker/GZ-Typst-Templates)，在 Unix 系统中使用 `ln -s /path/to/the/template/repo base` 创建一个名为 `base` 的软链接，目的是为了方便我们后续使用模板。操作成功后，你的目录结构应该类似于：

```txt
.
└── base -> ../GZ-Typst-Templates
```

!!! tip "对于 Windows 用户，可以考虑使用 mklink 来创建软链接，或直接拷贝模版文件"

考虑到可能有多个实验，推荐每个实验都新建一个文件夹，并创建对应的图片等目录，例如如下的结构

```txt
.
├── base -> ../GZ-Typst-Templates
└── lab-0
    ├── images
    │   └── linux.png
    └── example.typ
```

之后，在当前文件夹中打开终端或使用 VSCode，正常使用 typst 即可，例如：

!!! note "保证 typst 的工作路径为 `.`"

    引用其他更上层的目录是被 Typst 默认禁止的，这是为了避免[任意文件读取](https://github.com/typst/typst/issues/219)问题。

### 编写报告

打开 `example.typ`，并输入：

```js
#import "../base/templates/report.typ": *

#show: report.with(
  title: "实验报告实验",
  subtitle: "尝试使用 Typst 编写实验报告",
  name: "张三",
  stdid: "22223333",
  classid: "实验二班",
  major: "实验报告编写技术",
  school: "实验报告学院",
  time: "2022~2023 学年第二学期",
  banner: "../images/sysu.png"
)
```

如果安装了上文说的 Typst LSP 插件，那么此时应该能看到此时会自动生成一个 example.pdf，打开就能进行实时的预览。如果此时你没安装对应的字体的话，typst 会使用默认的字体来进行渲染，可能的话，请安装模板对应的字体。

!!! note "若需要修改字体，可以修改模板中的 `functions/style.typ` 中的 `fonts`，这里不做赘述。"

#### 标题

typst 在某些方面和 Markdown 有一定的相似之处，例如在 Markdown 中，我们使用 `#` 来表示标题，而在 typst 中，我们使用 `=` 来表示

即 Markdown 中的

```markdown
# 一级标题

## 二级标题

### 三级标题
```

可以简单的转换为

```text
= 一级标题

== 二级标题

=== 三级标题
```

#### 代码块

代码块也是相似的，基本上和 Markdown 的语法一致

即下面的 Markdown 也能直接被 typst 识别

````markdown
```c
#include <stdio.h>

int main() {
  printf("Hello World!\n");
  return 0;
}
```
````

#### 字体修改

可以使用如下的方法

```js
#text("Hello", font: "Times New Roman", size: 20pt, weight: "bold")
```

而对于加粗，还可以使用类似 Markdown 的 `*` 来括住需要强调

如果需要对大段文字字体进行修改，可以使用 `#set` 完成，这里不做展开

#### 图像

对于图像，格式为

```js
#image("/path/to/the/image")
```

#### 对齐方式

```js
#align(center)[
  这是一个居中样例
]
```

#### 表格

以如下的表格为例：

| 调度算法 | 算法描述 | 实验结果 |
| :------: | :------: | :------: |
|   FCFS   |    略    |    略    |
|   SSTF   |    略    |    略    |
|   SCAN   |    略    |    略    |

可以使用如下的例子

```text
#table(
  columns: (auto, auto, auto),
  align: center,
  [调度算法],[算法描述],[实验结果],
  [FCFS], [略], [略],
  [SSTF], [略], [略],
  [SCAN], [略], [略],
)
```

#### 数学公式

数学公式使用 `$` 括住即可，对于行间公式，在 `$` 和内容间加上空格，例如

```text
$ sum_(k=1)^n k = (n(n+1)) / 2 $
```

会被渲染成单独的一行：

$$ \sum\_{k=1}^n k = \frac{n(n+1)}{2} $$
