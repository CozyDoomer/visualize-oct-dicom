## Disclamer

I created this project to get to know rust a bit and it should not be considered usable or finished in any way.

## with release compiler optimizations 

`cargo run --release -- --max_image_size 512 --volume_path "/path/to/volume" --mask_paths "/path/to/mask1" "/path/to/mask2"`

## with rustup (could be used for bench feature in unstable) 

`cargo +nightly run --release -- --max_image_size 512 --volume_path "/path/to/volume" --mask_paths "/path/to/mask1" "/path/to/mask2"`
