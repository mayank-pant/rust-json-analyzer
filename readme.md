# Rust json parser

phase 1 - 
    take cli command as input
    input will be passed to lexer method, lexer method will convert input to list of tokens/in this case charcters
        and pass it to syntactic analysis step
    it will check if charcater is { then next charcater should be } or give error