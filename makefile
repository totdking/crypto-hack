venv : 
	@python3 -m venv venv

install: venv
	@venv/bin/pip install -r requirements.txt

act: 
	@. venv/bin/activate

run: 
	@venv/bin/python3 great_snakes_35381fca29d68d8f3f25c9fa0a9026fb.py
de: 
	@deactivate