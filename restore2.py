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
                    target_file = args.get('TargetFile', '')
                    
                    # Sometimes they are JSON-encoded strings, sometimes plain strings.
                    if target_file.startswith('"') and target_file.endswith('"'):
                        try:
                            target_file = json.loads(target_file)
                        except:
                            pass
                            
                    code_content = args.get('CodeContent', '')
                    if code_content.startswith('"') and code_content.endswith('"'):
                        try:
                            code_content = json.loads(code_content)
                        except:
                            pass

                    if "implementation_plan.md" in target_file or "task.md" in target_file or ".gitignore" in target_file:
                        continue
                    
                    if target_file.lower().startswith("c:/users/admin/documents/code_workspace/zenyt"):
                        os.makedirs(os.path.dirname(target_file), exist_ok=True)
                        with open(target_file, 'w', encoding='utf-8') as out:
                            out.write(code_content)
                        print(f"Restored {target_file}")
