#!/usr/bin/env python3
import re
import os
from pathlib import Path
from collections import defaultdict

SDK_PATH = os.environ.get('SDK')
FRAMEWORKS = ['Metal', 'MetalFX']

def extract_sdk_symbols():
    """Extract all public types from framework headers"""
    symbols = defaultdict(list)
    
    for fw in FRAMEWORKS:
        headers_path = f"{SDK_PATH}/System/Library/Frameworks/{fw}.framework/Headers"
        if not os.path.isdir(headers_path):
            continue
        
        for h_file in sorted(os.listdir(headers_path)):
            if not h_file.endswith('.h'):
                continue
            
            header_path = os.path.join(headers_path, h_file)
            with open(header_path) as f:
                content = f.read()
            
            # Skip unavailable symbols
            if 'API_UNAVAILABLE' in content or 'NS_UNAVAILABLE' in content:
                lines = content.split('\n')
                filtered = []
                skip_next = False
                for line in lines:
                    if 'API_UNAVAILABLE' in line or 'NS_UNAVAILABLE' in line:
                        skip_next = True
                    elif line.strip() and not line.startswith(' '):
                        skip_next = False
                    if not skip_next:
                        filtered.append(line)
                content = '\n'.join(filtered)
            
            # Extract @interface declarations
            for match in re.finditer(r'^@interface\s+(\w+)', content, re.MULTILINE):
                symbols[fw].append(('interface', match.group(1), h_file))
            
            # Extract @protocol declarations
            for match in re.finditer(r'^@protocol\s+(\w+)', content, re.MULTILINE):
                symbols[fw].append(('protocol', match.group(1), h_file))
            
            # Extract typedef enum/struct (simplified)
            for match in re.finditer(r'^typedef\s+(enum|struct|NS_ENUM|NS_OPTIONS)\s+\{.*?^\}?\s*(\w+)?', content, re.MULTILINE | re.DOTALL):
                name = match.group(2) or match.group(1)
                if name:
                    symbols[fw].append(('enum/struct', name, h_file))
    
    return symbols

# Extract and print counts
sdk_symbols = extract_sdk_symbols()
for fw, syms in sorted(sdk_symbols.items()):
    # Deduplicate
    unique = set((kind, name) for kind, name, _ in syms)
    print(f"{fw}: {len(unique)} symbols")
    for kind, name in sorted(unique)[:20]:
        print(f"  {kind:15} {name}")

print(f"\nTotal unique SDK symbols: {sum(len(set((k, n) for k, n, _ in syms)) for syms in sdk_symbols.values())}")
