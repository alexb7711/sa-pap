.PHONY = clean test long-test remote

##==============================================================================
#
all: ## Build and run the project
	@cargo build
	@cargo run

##==============================================================================
#
clean: ## Clean the project
	@cargo clean

##==============================================================================
#
test: ## Run tests
	@cargo test

##==============================================================================
#
remote-once: ## Run on remote server once (chuggy)
	@ssh chuggy "cd ~/sa-pap; git pull; pwd; screen -L -S sa-pap -md "/usr/bin/cargo run --release""

##==============================================================================
#
remote: ## Run on remote server 1000 times (chuggy)
	@ssh chuggy "cd ~/sa-pap; git pull; pwd; screen -L -S sa-pap -md "/usr/bin/cargo run 1000 --release""

##==============================================================================
#
long-test: ## Repeat tests 100 times
	for i in {0..100}; do \
		make test;          \
	done

##==============================================================================
# https://marmelab.com/blog/2016/02/29/auto-documented-makefile.html
help:  ## Auto-generated help menu
	@grep -P '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | \
	sort |                                                \
	awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'
