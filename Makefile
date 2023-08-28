

test-files-executable:
	find test-bin -type f -exec chmod a+x {} +
test: 
	cargo test -- --show-output 
