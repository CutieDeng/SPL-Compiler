%{
#include <stdio.h>
#include <stdlib.h>
int yylex(void);
void yyerror(const char *s) {
    // fprintf(stderr, "Error: %s\n", s);
}
%}

%token NUMBER
%left '+' '-'

%%
expr: expr '+' expr { printf("%d\n", $1 + $3); }
    | expr '-' expr { printf("%d\n", $1 - $3); }
    | NUMBER
    ;
%%

int main(void) {
    return yyparse();
}
