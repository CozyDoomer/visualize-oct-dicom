## with release compiler optimizations 

`cargo run --release -- --max_image_size 512 --volume_path "/run/media/christian/Extreme SSD/Projects/work/GENETECH_DUMMY/S" --mask_paths "/run/media/christian/Extreme SSD/Projects/work/GENETECH_DUMMY/S_HRF" "/run/media/christian/Extreme SSD/Projects/work/GENETECH_DUMMY/S_Fluid"`

## with rustup (could be used for bench feature in unstable) 

`cargo +nightly run --release -- --max_image_size 512 --volume_path "/run/media/christian/Extreme SSD/Projects/work/GENETECH_DUMMY/S" --mask_paths "/run/media/christian/Extreme SSD/Projects/work/GENETECH_DUMMY/S_HRF" "/run/media/christian/Extreme SSD/Projects/work/GENETECH_DUMMY/S_Fluid"`