# poll

## Usage

```
$ poll -c [command] [-i interval] [-l polling limit]
```

## Building

To build for release:

```
$ cargo build --release -p cli
```

To use on your command line, you must have the final binary in your `$PATH`. Here's an example of copying the release binary to your `$HOME/bin` directory.

If you haven't already, make the `bin` directory
```
$ mkdir ~/bin
```

From your `poll` project root directory, do the following to copy the release binary. Since the project is called `cli`, that is what binary called in the `target/release` directory. We are renaming it to `poll` when we copy to `~/bin`.
```
$ cp target/release/cli ~/bin/poll
```

Then be sure this is in your `.zshrc` or `.bashrc` file
```
export PATH=$HOME/bin:$PATH
```

Then remember to `source` the file
```
$ source ~/.zshrc
// of for bash shell
$ source ~/.bashrc
```

Then you should be able to call the `poll` binary with the following
```
$ poll -l 10 -i 2 -c "echo \"hello world\""
```
