# CNGREP(CG)

## Command
`cngrep [option] <pattern> <path>`
1. option(Support 1)
    1. --Count-Only, -c,    provide print search result number and only as first or last args.

2. pattern
    currently, support input to be searched word or number

3. path
    1. file, support single file like `content.txt`
    2. stdin, support recept pipe output like `zypper help`

## Example
1. Single File: `cngrep t content.txt`
2. Stdin: `zypper help | cngrep t`
3. Count Only: `cngrep -c t content.txt` or `cngrep t content.txt -c`