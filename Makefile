run-dev:
	cargo run

build:
	docker build -t weather-data-api .

run:
	docker run -p 8081:8080 weather-data-api

all: build run