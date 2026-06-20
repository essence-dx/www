import os
import re

json_files = [
    "app-route-discovery.json",
    "cache-manifest.json",
    "deploy-adapter.json",
    "forge-hosting-manifest.json",
    "hosted-preview.json",
    "import-resolution.json",
    "manifest.json",
    "observability.json",
    "provider-adapter-smoke-matrix.json",
    "provider-adapter.dx-cloud.json",
    "rollback.json",
    "route-handler-conformance-matrix.json",
    "route-handler-receipts.json",
    "server-action-replay-ledger.json",
    "source-build-manifest.json",
    "source-build-receipt.json"
]

def process_file(filepath):
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            content = f.read()
    except Exception as e:
        return

    original = content

    for json_f in json_files:
        pattern = r'(?<!\.dx/build-cache/)' + re.escape(json_f)
        content = re.sub(pattern, f'.dx/build-cache/{json_f}', content)

    content = content.replace('output_dir.join("source-routes")', 'output_dir.join(".dx/build-cache/source-routes")')
    content = content.replace('output_dir.join("source")', 'output_dir.join(".dx/build-cache/source")')
    content = content.replace('output_dir.join("image-placeholders")', 'output_dir.join(".dx/build-cache/image-placeholders")')
    
    content = content.replace('"source-routes/', '".dx/build-cache/source-routes/')
    content = content.replace('"image-placeholders/', '".dx/build-cache/image-placeholders/')

    if content != original:
        with open(filepath, 'w', encoding='utf-8') as f:
            f.write(content)

for root, dirs, files in os.walk(r'G:\Dx\www'):
    dirs[:] = [d for d in dirs if d not in ['target', 'node_modules', '.dx', '.git', 'android-cache', 'trash']]
    for file in files:
        if file.endswith('.rs') or file.endswith('.ts'):
            process_file(os.path.join(root, file))

print("Done")
