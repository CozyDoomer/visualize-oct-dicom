## with release compiler optimizations 

`cargo run --release -- --max_image_size 512 --volume_path "/path/to/volume" --mask_paths "/path/to/mask1" "/path/to/mask2"`

## with rustup (could be used for bench feature in unstable) 

`cargo +nightly run --release -- --max_image_size 512 --volume_path "/path/to/volume" --mask_paths "/path/to/mask1" "/path/to/mask2"`
