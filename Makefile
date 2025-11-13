.PHONY: help setup start stop restart logs health test clean

# Default target
help:
	@echo "LiteLLM Proxy Management Commands:"
	@echo ""
	@echo "  make setup      - Initial setup (create .env from template)"
	@echo "  make start      - Start all services"
	@echo "  make stop       - Stop all services"
	@echo "  make restart    - Restart all services"
	@echo "  make logs       - View logs (all services)"
	@echo "  make logs-lite  - View LiteLLM proxy logs only"
	@echo "  make health     - Check service health"
	@echo "  make test       - Test the proxy with a sample request"
	@echo "  make clean      - Stop and remove all containers and volumes"
	@echo "  make shell      - Open shell in LiteLLM container"
	@echo ""

# Initial setup
setup:
	@if [ ! -f .env ]; then \
		echo "Creating .env from .env.example..."; \
		cp .env.example .env; \
		echo ""; \
		echo "✓ Created .env file"; \
		echo ""; \
		echo "⚠️  IMPORTANT: Edit .env and add your API keys before running 'make start'"; \
		echo ""; \
	else \
		echo "⚠️  .env already exists, skipping..."; \
	fi

# Start all services
start:
	@echo "Starting LiteLLM proxy and dependencies..."
	docker-compose up -d
	@echo ""
	@echo "✓ Services started!"
	@echo ""
	@echo "LiteLLM Proxy: http://localhost:4000"
	@echo "Health check:  http://localhost:4000/health"
	@echo ""
	@echo "Run 'make logs' to view logs"
	@echo "Run 'make health' to check service health"

# Stop all services
stop:
	@echo "Stopping all services..."
	docker-compose down
	@echo "✓ Services stopped"

# Restart all services
restart:
	@echo "Restarting all services..."
	docker-compose restart
	@echo "✓ Services restarted"

# View logs
logs:
	docker-compose logs -f

# View LiteLLM logs only
logs-lite:
	docker-compose logs -f litellm

# Check health
health:
	@echo "Checking service health..."
	@echo ""
	@echo "Docker Compose Status:"
	@docker-compose ps
	@echo ""
	@echo "LiteLLM Health Check:"
	@curl -s http://localhost:4000/health | jq . || echo "Failed to connect to LiteLLM"

# Test the proxy
test:
	@echo "Testing LiteLLM proxy..."
	@echo ""
	@echo "1. Health check:"
	@curl -s http://localhost:4000/health | jq .
	@echo ""
	@echo "2. List models:"
	@curl -s http://localhost:4000/models \
		-H "Authorization: Bearer $$(grep LITELLM_MASTER_KEY .env | cut -d '=' -f2)" | jq .
	@echo ""
	@echo "3. Test completion (gpt-3.5-turbo):"
	@curl -s http://localhost:4000/v1/chat/completions \
		-H "Content-Type: application/json" \
		-H "Authorization: Bearer $$(grep LITELLM_MASTER_KEY .env | cut -d '=' -f2)" \
		-d '{"model": "gpt-3.5-turbo", "messages": [{"role": "user", "content": "Say hello"}]}' | jq .

# Clean up everything
clean:
	@echo "⚠️  This will remove all containers and volumes (including database data)"
	@read -p "Are you sure? [y/N] " -n 1 -r; \
	echo; \
	if [[ $$REPLY =~ ^[Yy]$$ ]]; then \
		docker-compose down -v; \
		echo "✓ Cleanup complete"; \
	else \
		echo "Cancelled"; \
	fi

# Open shell in LiteLLM container
shell:
	docker-compose exec litellm /bin/sh
