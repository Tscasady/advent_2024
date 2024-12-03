PROJECT_NAME=day$(X)
URL=https://adventofcode.com/2024/day/$(X)/input
COOKIE=_ga=GA1.2.685049924.1733076434; _gid=GA1.2.793680015.1733076434; session=53616c7465645f5f55c52831394e0f14519f4b72adfde80eef157c8b565fcb9bae3750fe12fea1e8695f2b98b00586a3c6e83cf4f4f8fc45a966d0aafb6eb0e0; _ga_MHSNPJKWC7=GS1.2.1733115911.11.0.1733115911.0.0.0

.PHONY: dayX

advent:
	@if [ -z "$(X)" ]; then echo "Error: Please provide X as an argument (e.g., make dayX X=1)"; exit 1; fi
	@echo "Creating project $(PROJECT_NAME)..."
	# Create a new Rust project
	cargo new $(PROJECT_NAME)
	# Navigate to the project directory
	@curl -sSL -H "Cookie: session=$(COOKIE)" $(URL) -o $(PROJECT_NAME)/input.txt

	@touch $(PROJECT_NAME)/test.txt
	@echo "Project $(PROJECT_NAME) setup complete."

