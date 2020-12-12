# Macrayon

A text preprocessor / macro system for [Crayon](https://crayonlang.org/).

## Building

Build with the latest stable [Rust](https://www.rust-lang.org/) using `cargo build --release`.

## Running

Run `macrayon` with no arguments to load macros from `MACROS` in the current directory then recursively copy every file of the form `*.macry` to `*.cry`, replacing macro calls with their instantiations. Alternatively, call `macrayon <macros file> <macrayon source> <crayon source>` to specify a macros file, a Macrayon source file, and the target Crayon source file.

## Examples

`MACROS`:
```
## vectorSum # v1 # v2 ## ([v1[0] + v2[0], v1[1] + v2[1]]) ##

## vectorDot # v1 # v1 ## (v1[0] * v2[0] + v1[1] * v2[1])) ##
```

`main.macry`:
```
function main() {
	v1 = [0, 1];
	v2 = [2, 3];
	dotProduct = ##vectorDot#v1#v2##;
	print(dotProduct);
}
```

Running `macrayon` generates `main.cry`:
```
function main() {
	vector1 = [0, 1];
	vector2 = [2, 3];
	dotProduct = (vector1[0] * vector2[0] + vector1[1] * vector2[1]));
	print(dotProduct);
}
```

## Why Should I Use Macrayon?

You shouldn't! 😃 But you might be tempted to use it anyway if you have lots of tiny Crayon function calls that are causing too much overhead.
