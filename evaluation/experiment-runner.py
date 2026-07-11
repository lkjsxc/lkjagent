#!/usr/bin/env python3

import sys

sys.dont_write_bytecode = True

from experiment_runner.campaign import main

if __name__ == "__main__":
    raise SystemExit(main())
