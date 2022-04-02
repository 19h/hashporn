<h1 align="center">hashporn</h1>

<h5 align="center">Detects resolution of videos, hashes them and renames accordingly.</h5>
<h5 align="center">hashporn hashes porn</h5>

<div align="center">
  <a href="https://crates.io/crates/hashporn">
    crates.io
  </a>
  —
  <a href="https://github.com/19h/hashporn">
    Github
  </a>
</div>

<br />

```shell script
$ cargo install hashporn
$ hashporn
```

<img src="https://i.imgur.com/0MOgSgL.png" />

#### What?

  - detects resolution of videos,
  - hashes them,
  - renames accordingly.

Imagine you have a bunch of videos, and you want to rename them according to their resolution.

There's a file called `vid12356000s0.mp4` in your directory.

If you run `hashporn`, it will rename the video to `2160-7f7640d154377f1a9466effd1b6c085e6eb63030.mp4`.

#### Why?

Certain kinds of connoisseurs have been known to use this tool to unify their collection.

#### Notes

`cargo` requires a rust installation.

#### License

~~ MIT License ~~

Copyright (c) 2022 Kenan Sulayman

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
