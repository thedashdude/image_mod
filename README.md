# image_mod
Rust program to apply the RGB, luma, and chroma distributions from one image to another.
## Execution

The program can be run from cargo using `cargo run file_source file_dest filters [files_out]`.
The output will be the filters applied to `file_dest` using distributions from `file_source` and saved to `out.png` or `files_out` if provided.

## Example

Starting with these two images, `example/sunset_small.png` and  `example/sprawl_small.png`:

![asdf](https://github.com/thedashdude/image_mod/blob/master/example/sunset_small.png)

![asdf](https://github.com/thedashdude/image_mod/blob/master/example/sprawl_small.png)

Reading the distribution of R,G,B, luma, and chroma values from `example/sunset_small.png` and applying those distributions to `example/sprawl_small.png` yields the following:

![asdf](https://github.com/thedashdude/image_mod/blob/master/example/sunset_on_sprawl.png)

The program will output the stats it read and the result of applying them to the image. Note that the application of distributions is imperfect, due to capping of RGB values at 255.

```
Loading files...
  Source: example\sunset_small.png
  Destination: example\sprawl_small.png



Running filters - RGBCL...
Filter 1/5 - R
Applying stat <RED>
  Source:
    Src Average <RED>: 132.03079
    Src Std.Dev <RED>: 71.97328
  Destination:
    Old Average <RED>: 82.72036
    Old Std.Dev <RED>: 64.97451
    New Average <RED>: 129.87796
    New Std.Dev <RED>: 68.82504



Filter 2/5 - G
Applying stat <GREEN>
  Source:
    Src Average <GREEN>: 89.17969
    Src Std.Dev <GREEN>: 44.88271
  Destination:
    Old Average <GREEN>: 81.00941
    Old Std.Dev <GREEN>: 63.9335
    New Average <GREEN>: 88.68812
    New Std.Dev <GREEN>: 44.88698



Filter 3/5 - B
Applying stat <BLUE>
  Source:
    Src Average <BLUE>: 92.86237
    Src Std.Dev <BLUE>: 31.229584
  Destination:
    Old Average <BLUE>: 77.72906
    Old Std.Dev <BLUE>: 67.66778
    New Average <BLUE>: 92.3486
    New Std.Dev <BLUE>: 31.236301



Filter 4/5 - C
Applying stat <CHROMA>
  Source:
    Src Average <CHROMA>: 52.184082
    Src Std.Dev <CHROMA>: 41.97763
  Destination:
    Old Average <CHROMA>: 48.93446
    Old Std.Dev <CHROMA>: 30.414684
    New Average <CHROMA>: 52.337437
    New Std.Dev <CHROMA>: 41.95156



Filter 5/5 - L
Applying stat <LUMA>
  Source:
    Src Average <LUMA>: 98.05054
    Src Std.Dev <LUMA>: 49.01451
  Destination:
    Old Average <LUMA>: 93.86577
    Old Std.Dev <LUMA>: 44.566185
    New Average <LUMA>: 97.154655
    New Std.Dev <LUMA>: 48.253654



Saving to 'example\sunset_on_sprawl.png'...
  Done```
