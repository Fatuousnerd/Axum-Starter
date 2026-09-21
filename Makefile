.PHONY: run dev test build docker-build docker-run migrate clean

run:
	cargo run

dev:
	cargo watch -c -x run

test:
	cargo test

build:
	cargo build --release

docker-build:
	docker build -t axum-starter -f src/Dockerfile

docker-run:
	docker run -p 7865:7865 --env-file .env axum-starter

migrate:
	cargo run --bin migrate

clean:
	cargo clean
	rm -f data.db sqlite.s

# Generate SQLx metadata (if using sqlx)
#sqlx-prepare:
#	cargo install
#	sqlx prepare --workspace