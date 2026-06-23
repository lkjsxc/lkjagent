#!/bin/bash

# Generate a cryptographically secure random password
# Usage: generate_random_password [length]
# Default length: 16 characters
# Requirements: at least one uppercase, lowercase, digit, and special character

set -euo pipefail

LENGTH=${1:-16}

if (( LENGTH < 4 )); then
    echo "Error: Password length must be at least 4" >&2
    exit 1
fi

if (( LENGTH > 32 )); then
    echo "Warning: Maximum recommended password length is 32 characters" >&2
fi

# Character sets for each required class
UPPERCASE=$(printf '%s' 'ABCDEFGHIJKLMNOPQRSTUVWXYZ')
LOWERCASE=$(printf '%s' 'abcdefghijklmnopqrstuvwxyz')
DIGITS=$(printf '%s' '0123456789')
SPECIAL=$(printf '%s' '!@#$%^&*()-_=+[]{}|;:,.<>?/~`')

# Ensure we have characters to draw from (fallback if empty)
[[ -n "$UPPERCASE" ]] || UPPERCASE="A"
[[ -n "$LOWERCASE" ]] || LOWERCASE="a"
[[ -n "$DIGITS" ]] || DIGITS="0"
[[ -n "$SPECIAL" ]] || SPECIAL="!"

# Generate the required characters (one from each class)
REQUIRED=""
REQUIRED+=$(head -c 1 /dev/urandom | tr -dc "$UPPERCASE" | head -c 1)
REQUIRED+=$(head -c 1 /dev/urandom | tr -dc "$LOWERCASE" | head -c 1)
REQUIRED+=$(head -c 1 /dev/urandom | tr -dc "$DIGITS" | head -c 1)
REQUIRED+=$(head -c 1 /dev/urandom | tr -dc "$SPECIAL" | head -c 1)

# Generate remaining characters from all character sets combined
ALL_CHARS="$UPPERCASE$LOWERCASE$DIGITS$SPECIAL"
FILL_LENGTH=$((LENGTH - ${#REQUIRED}))
if (( FILL_LENGTH > 0 )); then
    REQUIRED+=$(head -c $FILL_LENGTH /dev/urandom | tr -dc "$ALL_CHARS")
fi

# Shuffle the characters using fold and shuf for randomness
echo "$REQUIRED" | fold -w1 | shuf
