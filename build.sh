dist_dir="dist"
output="$(pwd)/$dist_dir"

echo "$output"
trunk build --release --dist "$output" --public-url "$dist_dir"
