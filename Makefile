DIST = target/
FRONTEND = www/

.PHONY: all
all: $(DIST)/release

# ------------------------
# 	BACKEND
# ------------------------

$(DIST)/release: Cargo.toml $(FRONTEND)dist
	cargo build --release

# ------------------------
# 	FRONTEND
# ------------------------

# Install the frontend dependencies
$(FRONTEND)node_modules: $(FRONTEND)
	@npm --prefix $(FRONTEND) install

# Build the frontend
$(FRONTEND)dist: $(FRONTEND)node_modules
	@npm --prefix $(FRONTEND) run build

.PHONY: frontend
frontend: $(FRONTEND)dist

.PHONY: frontend-dev
# Start the frontend in development mode
frontend-dev: $(FRONTEND)node_modules
	@npm --prefix $(FRONTEND) run dev

# ------------------------
# 	CLEAN
# ------------------------

.PHONY: purge
purge: prune

.PHONY: prune
prune: clean
	@rm -rf $(FRONTEND)node_modules

.PHONY: clean
clean:
	@rm -rf $(DIST) $(FRONTEND)dist