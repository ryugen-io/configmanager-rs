# Config Manager - Just commands

# Rebuild project (format, build backend with auditable, build frontend with trunk)
rebuild:
    ./rebuild.py

# Rebuild backend only
rebuild-backend:
    ./rebuild.py --backend-only

# Rebuild frontend only
rebuild-frontend:
    ./rebuild.py --frontend-only

# Rebuild without starting server
rebuild-no-server:
    ./rebuild.py --no-server

# Format all code
fmt:
    ./sys/rust/rustfmt.py

# Run clippy on all targets
clippy:
    ./sys/rust/clippy.py

# Run cargo check
check:
    ./sys/rust/check.py

# Run tests
test:
    ./sys/rust/test_rust.py

# Run security audit
audit:
    ./sys/rust/audit.py

# Clean build artifacts
clean:
    ./sys/rust/clean.py

# Start server (background process)
start:
    ./start.py

# Stop server
stop:
    ./stop.py

# Check server status
status:
    ./status.py

# View server logs
logs:
    tail -f server.log

# Count lines of code
loc:
    fish -c "locode rs"
