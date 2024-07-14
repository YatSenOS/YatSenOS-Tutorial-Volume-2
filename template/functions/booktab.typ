#import "helpers.typ": *
#import "fonts.typ": *

#let booktab(columns: (), aligns: (), width: auto, caption: none, ..cells) = {
  let headers = cells.pos().slice(0, columns.len())
  let contents = cells.pos().slice(columns.len(), cells.pos().len())
  set align(center)
  set text(size: 0.95em)

  if aligns == () {
    for i in range(0, columns.len()) {
      aligns.push(center)
    }
  }

  let content_aligns = ()
  for i in range(0, contents.len()) {
    content_aligns.push(aligns.at(calc.rem(i, aligns.len())))
  }

  figure(block(
    width: width,
    grid(columns: (auto), row-gutter: 0.6em, line(length: 100%), [
      #set align(center)
      #box(
        width: 100% - 1em,
        grid(columns: columns, ..zip(headers, aligns).map(it => [
          #set align(it.last())
          #textbf(it.first())
        ])),
      )
    ], line(length: 100%), [
      #set align(center)
      #box(width: 100% - 1em, grid(
        columns: columns,
        row-gutter: 1em,
        ..zip(contents, content_aligns).map(it => [
          #set align(it.last())
          #it.first()
        ]),
      ))
    ], line(length: 100%)),
  ), caption: caption, kind: table)
}
