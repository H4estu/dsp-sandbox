#!/usr/bin/env bash

for soundfile in $(ls -d ./test/sounds/*); do
    play $soundfile
done
