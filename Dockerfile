FROM rust:1.62

COPY . /wordle_helper
run cargo install --path /wordle_helper
CMD ["myapp"]

