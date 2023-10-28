%{
#include "syntax2.tab.h"
%}

%%
[0-9]+ { yylval = atoi(yytext); return NUMBER; }
"+" { return '+'; }
"-" { return '-'; }
\n { return 0; }
. { /* ignore other characters */ }
%%

int yywrap(void) {
    return 1;
}
