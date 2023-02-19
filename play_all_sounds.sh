#!/usr/bin/env zsh

for soundfile in $(ls -d ./test/sounds/*) do
    play $soundfile
done
