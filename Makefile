SHELL := /bin/bash
.DEFAULT_GOAL := help


NPM := npm
NPM_PREFIX := frontend
RUST_FRONTEND_STATIC_DIR := src/server/frontend/assets

web-build: ## build web frontend
	rm -rf $(NPM_PREFIX)/static $(RUST_FRONTEND_STATIC_DIR)
	$(NPM) run build --prefix $(NPM_PREFIX)
	cp -r $(NPM_PREFIX)/static $(RUST_FRONTEND_STATIC_DIR)

npm-test: ## run npm test
	$(NPM) run test --prefix $(NPM_PREFIX)

npm-dev: ## start npm dev server
	$(NPM) start --prefix $(NPM_PREFIX)

npm-lint: ## check lint node code
	$(NPM) run lint --prefix $(NPM_PREFIX)

npm-format: ## format node code
	$(NPM) run format --prefix $(NPM_PREFIX)

# https://gist.github.com/tadashi-aikawa/da73d277a3c1ec6767ed48d1335900f3
.PHONY: $(shell grep -h -E '^[a-zA-Z_-]+:' $(MAKEFILE_LIST) | sed 's/://')

# https://postd.cc/auto-documented-makefile/
help: ## Show help message
	@grep -h -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'
