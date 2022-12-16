# runn a command inside of a nix-shell with the required pkgs, command must be in quotes
NIX_LIBS := \
	libiconv \
	openssl \
	pkgconfig \
	pkgs.darwin.apple_sdk.frameworks.Security \
	zlib \

NIX_SHELL_RUN := nix-shell -p ${NIX_LIBS} --run

build:
	${NIX_SHELL_RUN} 'cargo build'

run:
	${NIX_SHELL_RUN} 'cargo run'
