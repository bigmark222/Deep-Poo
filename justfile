default: help

help:
	@echo "Available recipes:"
	@just --list

fmt:
	cargo fmt --all

run-sim:
	cargo run --locked --bin sim_view -- $(ARGS)

run-infer:
	cargo run --locked --bin inference_view -- $(ARGS)

smoke:
	cargo test --locked --test registry_smoke

# Headless datagen smoke with overrides
smoke-datagen:
	cargo run --locked --bin datagen_headless -- --max-frames 3 --headless --output-root /tmp/captures $(ARGS)
