default:
	@just --list

# Auto-format the source tree
fmt:
	treefmt

# Run 'cargo run' on the project
run *ARGS:
	just dont-fuck-my-build
	cargo run {{ARGS}}

# Run 'cargo watch' to run the project (auto-recompiles)
watch *ARGS:
	cargo watch -x "run -- {{ARGS}}"

# Show the current state of the project
show:
	just dont-fuck-my-build
	just om show .

# Ensure no untracked or uncommitted .nix files are left out
dont-fuck-my-build:
	git ls-files --others --exclude-standard -- '*.nix' | xargs -r git add -v | lolcat
	echo "No chance your build is fucked! 👍" | lolcat

# Run the 'omnix' tool with the provided arguments
om *ARGS:
	nix run github:juspay/omnix -- {{ ARGS }}

# Check the health of the project
health:
	just dont-fuck-my-build
	just om health .

# Clean up build artifacts and temporary files
clean:
	rm -rfv result
	cargo clean
	quick-results

# Update a single flake input using a nice little tool created by vimjoyer
single-update:
	nix run github:vimjoyer/nix-update-input

# Update dependencies and the Nix flake lock file, committing the changes
update:
	just dont-fuck-my-build
	cargo-update
	nix flake update --commit-lock-file

# Update dependencies and the Nix flake lock file without committing the changes
update-no-commit:
	just dont-fuck-my-build
	cargo-update --no-commit
	nix flake update

# Update only the Nix flake lock file, committing the changes
update-flake:
	nix flake update --commit-lock-file

# Update only the Nix flake lock file without committing the changes
update-flake-no-commit:
	nix flake update

# Build the project using Nix with the provided arguments
build *ARGS:
	nix build {{ ARGS }}
	quick-results

# Check the project using Nix flake and other tools
check *ARGS:
	just dont-fuck-my-build
	nix flake check --impure --no-build {{ ARGS }}
	nix-shell -p lolcat --run 'echo "[CHECK] Finished." | lolcat'
