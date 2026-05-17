#!/bin/bash

# Extract all public interface/protocol/enum/struct from Metal and MetalFX

SDK_PATH="${SDK}"

echo "=== METAL AND METALFX SDK PUBLIC SURFACE ==="

for FRAMEWORK in Metal MetalFX; do
  HEADERS_PATH="$SDK_PATH/System/Library/Frameworks/$FRAMEWORK.framework/Headers"
  
  if [ ! -d "$HEADERS_PATH" ]; then
    echo "Framework $FRAMEWORK not found"
    continue
  fi
  
  echo ""
  echo "### $FRAMEWORK FRAMEWORK ###"
  
  # Extract @interface, @protocol definitions
  grep -h "^@interface\|^@protocol" "$HEADERS_PATH"/*.h 2>/dev/null | \
    grep -v "API_UNAVAILABLE\|NS_UNAVAILABLE\|__OSX_UNAVAILABLE\|__deprecated\|__attribute__((deprecated))" | \
    sed 's/ {$//' | sort -u
  
  # Extract typedef enums and structs
  grep -h "^typedef enum\|^typedef struct\|^typedef NS_ENUM\|^typedef NS_OPTIONS" "$HEADERS_PATH"/*.h 2>/dev/null | \
    grep -v "API_UNAVAILABLE\|NS_UNAVAILABLE\|__OSX_UNAVAILABLE\|deprecated" | sort -u
done
