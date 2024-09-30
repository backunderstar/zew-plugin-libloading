
PROJECT_NAME := zew
WORKSPACES := $(shell find ./plugin -name Cargo.toml -exec dirname {} \;)
WORKSPACE_NAMES := $(foreach dir,$(WORKSPACES),$(notdir $(dir)))
CARGO := cargo
DIST_DIR := dist

.DEFAULT_GOAL := help

.PHONY: build run test clean plugin plugin-dev release dev

help:
	@echo "Available targets:"
	@echo "  build       - Build the project"
	@echo "  run         - Run the project"
	@echo "  test        - Run tests"
	@echo "  dev         - Build and run the project"
	@echo "  clean       - Clean the target directory"
	@echo "  plugin      - Build the plugin"
	@echo "  release     - Build the release version"

build:
	$(CARGO) build --release

plugin:
	@for workspace in $(WORKSPACE_NAMES); do \
		echo "Building $$workspace";  \
		$(CARGO) build --release -p $$workspace; \
		if [ -f "target/release/$$workspace.so" ]; then \
    	cp target/release/$$workspace.so plugin/$$workspace/$$workspace.so; \
		elif [ -f "target/release/$$workspace.dll" ]; then \
    	cp target/release/$$workspace.dll plugin/$$workspace/$$workspace.dll; \
		else \
            echo "No library file found for $$workspace"; \
        fi; \
	done

plugin-dev:
	@for workspace in $(WORKSPACE_NAMES); do \
		echo "Building $$workspace";  \
		$(CARGO) build -p $$workspace; \
		if [ -f "target/debug/$$workspace.so" ]; then \
    	cp target/debug/$$workspace.so plugin/$$workspace/$$workspace.so; \
		elif [ -f "target/debug/$$workspace.dll" ]; then \
    	cp target/debug/$$workspace.dll plugin/$$workspace/$$workspace.dll; \
		else \
            echo "No library file found for $$workspace"; \
        fi; \
	done

run:
	$(CARGO) run 

test:
	$(CARGO) test

dev: plugin-dev run 

pro: plugin 
	$(CARGO) run --release

clean:
	@rm -rf target/release 
	@rm -rf target/debug
	@rm -rf dist
	@rm -rf plugin/**/*.so 
	@rm -rf plugin/**/*.dll 

release: build plugin
	@if [ ! -d $(DIST_DIR) ]; then mkdir -p $(DIST_DIR); \
		echo "Created $(DIST_DIR)"; \
	fi 
	@if [ -f "target/release/$(PROJECT_NAME).exe" ]; then \
		cp target/release/$(PROJECT_NAME).exe $(DIST_DIR)/$(PROJECT_NAME).exe; \
		echo "Copied $(PROJECT_NAME).exe"; \
	elif [ -f "target/release/$(PROJECT_NAME)" ]; then \
		cp target/release/$(PROJECT_NAME) $(DIST_DIR)/$(PROJECT_NAME); \
		echo "Copied $(PROJECT_NAME)"; \
	else \
		echo "No binary file found"; \
	fi
	@for workspace in $(WORKSPACE_NAMES); do \
		target_dir=$(DIST_DIR)/plugin/$$workspace; \
		if [ ! -d "$$target_dir" ]; then mkdir -p $$target_dir; \
			echo "Created $$target_dir"; \
		fi; \
		if [ -f "plugin/$$workspace/$$workspace.so" ]; then \
			cp plugin/$$workspace/$$workspace.so $$target_dir/$$workspace.so; \
			echo "Copied $$workspace.so"; \
		elif [ -f "plugin/$$workspace/$$workspace.dll" ]; then \
			cp plugin/$$workspace/$$workspace.dll $$target_dir/$$workspace.dll; \
			echo "Copied $$workspace.dll"; \
		else \
			echo "No library file found for $$workspace"; \
		fi; \
	done
	@if [ -f ".env" ]; then \
		cp .env $(DIST_DIR)/.env; \
		echo "Copied .env file"; \
	fi