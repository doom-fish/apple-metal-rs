#!/bin/bash

# Extract all Metal and MetalFX public types with details

SDK_PATH="${SDK}"

echo "=== METAL PUBLIC TYPES ==="
for fw in Metal MetalFX; do
  headers_path="$SDK_PATH/System/Library/Frameworks/$fw.framework/Headers"
  
  echo "# $fw Framework"
  
  # Extract @interface/@protocol
  grep -h "^@interface\|^@protocol" "$headers_path"/*.h 2>/dev/null | \
    grep -v "API_UNAVAILABLE\|NS_UNAVAILABLE\|__OSX_UNAVAILABLE" | \
    sed 's/^@interface \([^ <:]*\).*/TYPE: \1/' | \
    sed 's/^@protocol \([^ <;]*\).*/TYPE: \1/' | \
    sort -u | head -30
  
  echo ""
done
