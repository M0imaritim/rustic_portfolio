#!/bin/bash

# Fix Leptos 0.7 class attributes - only change class to attr:class for <A> components

for file in src/components/*; do
    # Revert all attr:class back to class
    sed -i 's/attr:class=/class=/g' "$file"

    # Then, only change class to attr:class specifically for <A> components
    # This matches <A href="..." class="..." and changes class to attr:class
    sed -i 's/<A \([^>]*\)class=/<A \1attr:class=/g' "$file"
done

echo "Fixed all files."
