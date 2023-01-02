# Cleaner

This small tool recursively searches a directory for audio files, images and
other files, or archives containing audiofiles, and then scans those audio
files to check if they contain necessary tags (for mp3 or flac files). If tags
are missing, automatic corrections are made.

This is a work in progress project used to help learn the Rust programming
language.

As such, there is likely non-idiomatic code in here, beware!

## Main Features

 - Scan a directory recursively for audio files and cover art
 - Scan a directory for archives (zip or rar), unpack those archives to scan
   audio files contained therein
 - Check audio tags and, where possible, automatically fill in missing tags,
   add missing cover art
 - Rename files to match a standard pattern
 - Original files are preserved
