shell:
	@cargo run -- shell

server:
	@cargo run -- server

container:
	@docker run -p 9876:9876 ferrodb

test:
	@./test.sh