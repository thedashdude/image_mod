# image_mod
Rust program to apply the luma and chroma distributions from one image to another.
## Execution

The program can be run from cargo using `cargo run file_source file_dest`.
The output will be the stats of `file_source` applied to `file_dest` and saved to `out.png`.

## Example

Starting with these two images, `example/waterfall_small.png` and  `example/waterfall_small.png`:

![asdf](https://github.com/thedashdude/image_mod/blob/master/example/sunset_small.png)

![asdf](https://github.com/thedashdude/image_mod/blob/master/example/waterfall_small.png)

Reading the distribution of luma and chroma values from `example/waterfall_small.png` and applying those distributions to `example/waterfall_small.png` yields the following:

![asdf](https://github.com/thedashdude/image_mod/blob/master/example/sunset_on_waterfall.png)

Note that the *hue* hasn't changed, only the vibrancy (chroma) of the colors.

The program will output the stats it read and the result of applying them to the image. Note that the application is imperfect, due to capping of RGB values at 255.

![asdf](https://github.com/thedashdude/image_mod/blob/master/example/output.png)

