govenv:
	source venv/bin/activate

install:
	pip install -r requirements.txt

docker:
	docker build --platform linux/amd64 -t bugarin10/midsmix:latest .