#!/usr/bin/env python3

import subprocess
import fileinput

subprocess.run(["zellij", "action", "focus-next-pane"])

args = []
for line in fileinput.input():
    for b in bytes(line.rstrip(), 'utf-8'):
        args.append(str(b))
    args.append(str(13))

subprocess.run(["zellij", "action", "write", *args])
subprocess.run(["zellij", "action", "focus-next-pane"])
