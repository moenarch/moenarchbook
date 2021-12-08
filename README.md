# moenarchbook

moenarchbook is a utility to create modern online books from Markdown files.


## What does it look like?

The [User Guide] for moenarchbook has been written in Markdown and is using moenarchbook to
generate the online book-like website you can read. The documentation uses the
latest version on GitHub and showcases the available features.

## Installation

There are multiple ways to install moenarchbook.

1. **Binaries**

   Binaries are available for download [here][releases]. Make sure to put the
   path to the binary into your `PATH`.

2. **From Crates.io**

   This requires at least [Rust] 1.39 and Cargo to be installed. Once you have installed
   Rust, type the following in the terminal:

   ```
   cargo install moenarchbook
   ```

   This will download and compile moenarchbook for you, the only thing left to do is
   to add the Cargo bin directory to your `PATH`.

   **Note for automatic deployment**

   If you are using a script to do automatic deployments using Travis or
   another CI server, we recommend that you specify a semver version range for
   moenarchbook when you install it through your script!

   This will constrain the server to install the latest **non-breaking**
   version of moenarchbook and will prevent your books from failing to build because
   we released a new version.

   You can also disable default features to speed up compile time.

   Example:

   ```
   cargo install moenarchbook --no-default-features --features output --vers "^0.1.0"
   ```

3. **From Git**

   The version published to crates.io will ever so slightly be behind the
   version hosted here on GitHub. If you need the latest version you can build
   the git version of moenarchbook yourself. Cargo makes this ***super easy***!

   ```
   cargo install --git https://github.com/moenarch/moenarchbook.git moenarchbook
   ```

   Again, make sure to add the Cargo bin directory to your `PATH`.

## Usage

moenarchbook is primarily used as a command line tool, even though it exposes
all its functionality as a Rust crate for integration in other projects.

Here are the main commands you will want to run. For a more exhaustive
explanation, check out the [User Guide].

- `moenarchbook init <directory>`

    The init command will create a directory with the minimal boilerplate to
    start with. If the `<directory>` parameter is omitted, the current 
    directory will be used.

    ```
    book-test/
    ├── book
    └── src
        ├── chapter_1.md
        └── SUMMARY.md
    ```

    `book` and `src` are both directories. `src` contains the markdown files
    that will be used to render the output to the `book` directory.

    Please, take a look at the [CLI docs] for more information and some neat tricks.

- `moenarchbook build`

    This is the command you will run to render your book, it reads the
    `SUMMARY.md` file to understand the structure of your book, takes the
    markdown files in the source directory as input and outputs static html
    pages that you can upload to a server.

- `moenarchbook watch`

    When you run this command, moenarchbook will watch your markdown files to rebuild
    the book on every change. This avoids having to come back to the terminal
    to type `moenarchbook build` over and over again.

- `moenarchbook serve`

    Does the same thing as `moenarchbook watch` but additionally serves the book at
    `http://localhost:3000` (port is changeable) and reloads the browser when a
    change occurs.

- `moenarchbook clean`

    Delete directory in which generated book is located.

## License

All the code in this repository is released under the ***Mozilla Public License v2.0***, for more information take a look at the [LICENSE] file.
