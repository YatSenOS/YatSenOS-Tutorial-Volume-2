#import "./functions/fonts.typ": *
#import "./functions/booktab.typ": *
#import "@preview/physica:0.8.0": *

// Set the default style for the document
#let shared(doc) = {
  set page(paper: "a4", numbering: "1", number-align: center)
  set text(font: fonts.serif, lang: "zh", size: 11pt)

  show raw.where(block: true): block.with(fill: luma(240), inset: (x: 1.25em, y: 1em), width: 100%, radius: 4pt)

  show raw.where(block: true): par.with(first-line-indent: 0em, justify: true, leading: 8pt)

  show raw.where(block: false): box.with(
    fill: luma(240),
    inset: (x: 5pt, y: 0pt),
    outset: (y: 4pt),
    radius: 3pt,
  )

  show raw: text.with(font: fonts.monospace + fonts.sans, size: 1em)

  set heading(numbering: (..numbers) => {
    let level = numbers.pos().len();

    if (level == 1) {
      return numbering("一. ", numbers.pos().at(level - 1))
    } else {
      return numbering("1.1. ", ..numbers.pos().slice(1, level))
    }
  })

  show heading: it => [
    // Cancel indentation for headings of level 2 or above
    #set par(first-line-indent: 0em, hanging-indent: 2em)

    #let sized_heading(it, size, weight, mt, mb) = {
      set text(size, weight: weight)
      v(mt)
      counter(heading).display()
      text[#it.body]
      v(mb)
    }

    #if it.level == 1 {
      sized_heading(it, 20pt, "semibold", 1em, 0.6em)
    } else if it.level == 2 {
      sized_heading(it, 16pt, "semibold", 0.7em, 0.7em)
    } else if it.level == 3 {
      sized_heading(it, 14pt, "medium", 0.5em, 0.5em)
    } else {
      sized_heading(it, 12pt, "medium", 0.3em, 0.3em)
    }
  ]

  show link: underline
  set math.vec(delim: "[")
  set math.mat(delim: "[")
  show par: set block(spacing: line_height)

  doc
}
