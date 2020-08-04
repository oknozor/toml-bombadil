# TODO 

- install :

1. remove preexisting `.dot` dir

2. create `.dot`

1. variables
    - Read vars from config path
    - Read colors from config path
    - Aggregate to Variable(Hasmap) struct
    - Greedy parsing on dotfiles

4. copy dotfiles to `.dots` with template vars injected

5. run post install hooks 

- git
    - add .dots to .gitignore if exists
    

- list command : to get every symlinked file in a treeish style (source and target)
- theme command: to print  a list of available themes (with  hue sorted colors)
- theme set command: set the current theme 
- vim syntax for everything insinde dotfiles/ (see :https://vim.fandom.com/wiki/Creating_your_own_syntax_files)