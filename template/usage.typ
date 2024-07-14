#import "./report.typ": *

#show: report.with(
  title: "实验报告实验",
  subtitle: "尝试使用 Typst 编写实验报告",
  name: "张三",
  stdid: "22223333",
  classid: "实验二班",
  major: "实验报告编写技术",
  school: "实验报告学院",
  time: "2022~2023 学年第二学期",
  banner: "./images/sysu.png"
)

#title(content: "尝试使用 Typst 编写实验报告")

= 实验报告实验

== 什么是实验报告

#lorem(200)

#booktab(
  width: 100%,
  aligns: (left + horizon, center, right + bottom),
  columns: (1fr, 1fr, 1fr),
  caption: [`booktab` 示例],
  [左对齐], [居中], [右对齐],
  [4], [5], [6],
  [7], [8], [9],
  [10], [$ (n(n+1)) / 2 $], [11],
)

=== 公式

$ sum_(k=0)^n k
    &= 1 + ... + n \
    &= (n(n+1)) / 2 $

== 什么是 Typst

#lorem(20)

行内代码块： `#lorem(20)`



```c
int main() {
  // 这是一句中文注释
  // This is a comment in English
  printf("Hello, world!");
  return 0;
}
```

== 做个数学题

#set par(first-line-indent: 0pt)

3.1 设 $cal(V)$ 和 $cal(W)$ 是两 Hilbert 空间，${ket(v_j)}$ 和 ${ket(w_j)}$ 分别为 $cal(V)$ 和 $cal(W)$ 的一组正交模基。设 $display(T_A: sum_(j = 1)^n c_j ket(v_j) arrow.r sum_(j=1)^n c_j sum_(i=1)^m a_(i j) ket(w_j))$，证明 $T_A$ 是线性算子。

证明：对于 $forall k in NN, 0 < k <= n$，可以取一组 $display(c_j = cases(
    1 "," j = k,
    0 "," j != k
))$，由上述定义，可得到：

$ T_A (sum_(j = 1)^n c_j ket(v_j)) &= T_A (c_k ket(v_j)) = T_A (ket(v_j)) \
&= sum_(j=1)^n c_j sum_(i=1)^m a_(i j) ket(w_j) = c_k sum_(i=1)^m a_(i j) ket(w_j) = sum_(i=1)^m a_(i j) ket(w_j) $

则对于任意的一组 $c_j$，可以得到：

$ T_A (sum_(j = 1)^n c_j ket(v_j)) = sum_(j=1)^n c_j sum_(i=1)^m a_(i j) ket(w_j) = sum_(j=1)^n c_j T_A (ket(v_j)) $

即 $T_A$ 是线性算子。
