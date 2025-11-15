#!/bin/bash
# Line counter script for Config Manager
# Counts lines of code in Rust files, excluding comment-only lines

set -e
set -o pipefail

# Configuration
readonly SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

# Catppuccin Mocha color palette (24-bit true color)
readonly RED='\033[38;2;243;139;168m'        # #f38ba8 - Errors
readonly GREEN='\033[38;2;166;227;161m'      # #a6e3a1 - Success/Info
readonly YELLOW='\033[38;2;249;226;175m'     # #f9e2af - Warnings
readonly BLUE='\033[38;2;137;180;250m'       # #89b4fa - Info highlights
readonly MAUVE='\033[38;2;203;166;247m'      # #cba6f7 - Headers
readonly SAPPHIRE='\033[38;2;116;199;236m'   # #74c7ec - Success highlights
readonly TEXT='\033[38;2;205;214;244m'       # #cdd6f4 - Normal text
readonly NC='\033[0m'                         # No Color

# Logging functions
log_info() {
    echo -e "${BLUE}  ${NC}$1"
}

log_success() {
    echo -e "${SAPPHIRE}  ${NC}$1"
}

# Function to count lines in a file, excluding comments
count_lines() {
    local file="$1"

    # Count total lines
    local total=$(wc -l < "$file")

    # Count comment-only lines (lines that start with optional whitespace followed by // or ///)
    # This excludes:
    # - Lines starting with // (regular comments)
    # - Lines starting with /// (doc comments)
    # But includes:
    # - Lines with code before // (inline comments)
    # - Empty lines
    local comments=$(grep -cE '^\s*(//|///)' "$file" || true)

    # Count blank lines
    local blank=$(grep -cE '^\s*$' "$file" || true)

    # Code lines = total - comments - blank
    local code=$((total - comments - blank))

    echo "$code $comments $blank $total"
}

# Main script
echo -e "${MAUVE}[lines]${NC} Counting lines of code in Rust files..."
echo ""

total_code=0
total_comments=0
total_blank=0
total_lines=0
file_count=0

# Find all .rs files
while IFS= read -r -d '' file; do
    # Skip target directory
    if [[ "$file" == *"/target/"* ]]; then
        continue
    fi

    read -r code comments blank total <<< "$(count_lines "$file")"

    total_code=$((total_code + code))
    total_comments=$((total_comments + comments))
    total_blank=$((total_blank + blank))
    total_lines=$((total_lines + total))
    file_count=$((file_count + 1))
done < <(find "$SCRIPT_DIR" -name "*.rs" -type f -print0)

# Calculate percentages
code_pct=$(awk "BEGIN {printf \"%.1f\", ($total_code / $total_lines) * 100}")
comment_pct=$(awk "BEGIN {printf \"%.1f\", ($total_comments / $total_lines) * 100}")
blank_pct=$(awk "BEGIN {printf \"%.1f\", ($total_blank / $total_lines) * 100}")

# Print summary
echo -e "${GREEN}Summary:${NC}"
echo "─────────────────────────────────────"
printf "Files:          %6d\n" "$file_count"
printf "Code lines:     %6d (${TEXT}%s%%${NC})\n" "$total_code" "$code_pct"
printf "Comment lines:  %6d (${TEXT}%s%%${NC})\n" "$total_comments" "$comment_pct"
printf "Blank lines:    %6d (${TEXT}%s%%${NC})\n" "$total_blank" "$blank_pct"
printf "${YELLOW}Total lines:    %6d${NC}\n" "$total_lines"
echo "─────────────────────────────────────"
echo ""
log_info "Comment lines include lines starting with ${TEXT}//${NC} or ${TEXT}///${NC}"
log_info "Inline comments (code followed by //) are counted as code"
echo ""
log_success "Line count complete!"
