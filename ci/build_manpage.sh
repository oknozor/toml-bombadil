#!/usr/bin/env bash
set -euo pipefail

pandoc man/bombadil.1.md -s -t man
