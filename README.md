<h1 style="text-align: center;">NES Emulator</h1>
<h2 style="text-align: center;">NES Emulator written in Rust</h2>

The goal of this project is to write an Entire NES Emulator in Rust. This looks different, because this is the second time I'm implementing this.


<h2 style="text-align: center;">Why the rewrite?</h2>

There are two main reasons for the rewrite:

- Commit history: If you look at the previous commits, they're unreadable, make no sense. I'm not proud of it, but you live n learn
    - no, I'm not going to reword 100 commits, especially when each commit has multiple changes
- There are a lot of problems with the NES Code:
    - Various bugs: ppu rendering wasn't accurate
    - Lack of a logging system, making troubleshooting impossible.

<h2 style="text-align: center;">Steps</h2>

Writing an Emulator is a time intensive process, hence It'll take time.