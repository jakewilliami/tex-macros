# [DEPRECATED] mktex

**[DEPRECATED] This project has moved to [jakewilliami/mktex](https://github.com/jakewilliami/mktex)**

---

For a while, I've had a script [`mktex`](../mktex).  However, it is written in Bash, so it's very big and not super portable/reliable.  I figured, now that I am back at uni and using LaTeX, it would be a good time to start to port some of the functionality (and perhaps some more complex functionality) of `mktex` to a more reliable-in-production language.  I chose Rust.

The main thing we want to extend is the class option, which generates an empty LaTeX document with a given class.  However, we no longer want this class to be local to each project, so we should put it into the local texmf folder.  Along with this, version controlling becomes more important, so we may want to implement features that allow us to evaluate the class in a single file at a given time.

Finally, it would also be nice to have this pull from the git remote, rather than assuming local files exist.
