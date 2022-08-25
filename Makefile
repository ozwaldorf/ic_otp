.PHONY: replica stop-replica candid local clean

replica:
	dfx ping &>/dev/null || dfx start --clean --background

stop-replica:
	dfx stop

candid:
	cargo test candid

local: candid replica
	dfx deploy

ic: candid
	dfx deploy --network ic

clean: stop-replica
	cargo clean