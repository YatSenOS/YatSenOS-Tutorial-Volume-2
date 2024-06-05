#import "./shared.typ": *

#let report(
  title: "实验报告",
  subtitle: "Lab 0: 环境准备",
  name: "张三",
  stdid: "11223344",
  classid: "计科一班",
  major: "计算机科学与技术",
  school: "计算机学院",
  time: "2022~2023 学年第二学期",
  banner: none + "",
  body,
) = {
  set document(title: title)

  show: shared

  let fieldname(name) = [
    #set align(right + horizon)
    #set text(font: fonts.serif)
    #name
  ]

  let fieldvalue(value) = [
    #set align(left + horizon)
    #set text(font: fonts.serif)
    #value
  ]

  align(center)[
    #if (banner == none) {
      v(200pt)
    } else {
      image(banner, height: 200pt, width: 95%, fit: "contain")
    }
    #linebreak()
    #v(50pt, weak: true)
    #block(text(weight: "semibold", 30pt, title))
    #v(50pt, weak: true)
    #block(text(weight: "medium", 24pt, subtitle))

    #v(140pt, weak: true)
    #set text(14.5pt)

    #grid(
      columns: (200pt, 240pt),
      row-gutter: 1em,
      fieldname(text("姓") + h(2em) + text("名：") + h(1em)),
      fieldvalue(name),
      fieldname(text("学") + h(2em) + text("号：") + h(1em)),
      fieldvalue(stdid),
      fieldname(text("教学班号：") + h(1em)),
      fieldvalue(classid),
      fieldname(text("专") + h(2em) + text("业：") + h(1em)),
      fieldvalue(major),
      fieldname(text("院") + h(2em) + text("系：") + h(1em)),
      fieldvalue(school),
    )

    #v(40pt)
    #set text(13pt)
    #text(time)
    #pagebreak()
  ]

  set align(left + top)
  set par(justify: true, first-line-indent: 2em, leading: line_height)

  body
}

#let title(content: "") = align(center)[
  #set text(26pt, weight: "bold")
  #v(2em)
  #content
  #v(1.625em)
]
