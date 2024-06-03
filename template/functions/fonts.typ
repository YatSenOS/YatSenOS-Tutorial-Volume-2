#let line_height = 1em

#let fonts = (
  serif: (
    "Source Han Serif SC",
    "Source Han Serif",
    "宋体",
    "Times New Roman",
    "Georgia",
    "Garamond",
    "Palatino",
    "Baskerville",
  ),
  sans: (
    "Source Han Sans SC",
    "Source Han Sans",
    "黑体",
    "SimHei",
    "Microsoft YaHei",
    "PingFang SC",
  ),
  monospace: ("JetBrains Mono", "Courier New", "Courier", "Consolas", "Monaco"),
)

#let textbf(it) = block(text(font: fonts.sans, weight: "semibold", it))

#let textit(it) = block(text(style: "italic", it))
