import json
import os

transcript_path = r"C:\Users\Admin\.gemini\antigravity\brain\d5e7abc0-e943-4723-bf58-c0eb22aff975\.system_generated\logs\transcript_full.jsonl"

with open(transcript_path, 'r', encoding='utf-8') as f:
    for line in f:
        data = json.loads(line)
        if 'tool_calls' in data:
            for call in data['tool_calls']:
                if call['name'] == 'write_to_file':
                    args = call['args']
                    if 'TargetFile' in args and 'CodeContent' in args:
                        target_file = json.loads(args['TargetFile'])
                        # Only restore docs, readme, etc. Ignore .gitignore since Tauri has one.
                        if "implementation_plan.md" in target_file or "task.md" in target_file:
                            continue
                        if ".gitignore" in target_file:
                            continue
                        
                        if target_file.startswith("c:/Users/Admin/Documents/CODE_WORKSPACE/ZenYT"):
                            content = json.loads(args['CodeContent'])
                            os.makedirs(os.path.dirname(target_file), exist_ok=True)
                            with open(target_file, 'w', encoding='utf-8') as out:
                                out.write(content)
                            print(f"Restored {target_file}")
