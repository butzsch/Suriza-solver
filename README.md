Suriza solver
===

Detecting a [Suriza](https://de.wikipedia.org/wiki/Suriza) puzzle from a piece of paper, solving it and drawing the solution back onto the puzzle. A working example can be seen in the video:

[![Video of machine solving Suriza puzzle](https://i.imgur.com/c8dqKhK.png)](https://www.youtube.com/watch?v=CDgQVnWAoVk "suriza solved")

---

The project is split into multiple parts. The initial image processing part of the program is written in Python using OpenCV and Tesseract. After detecting the digits it then calls into Rust code which solves the puzzle and calculates the movements the pen needs to make to draw the solution onto the paper. The coordinates of the path are then transformed into G-codes and sent to an Arduino running GRBL.
