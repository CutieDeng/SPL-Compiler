%{

#include "string.h"
#include "stdlib.h"
_Bool has_error; 

%}
%{

int yycolno = 1;
#define YY_USER_ACTION_0 do { \
    yylloc.first_line = yylineno; \
    yylloc.first_column = yycolno; \
    yylloc.last_line = yylineno; \
    yylloc.last_column = yycolno + yyleng; \
    yycolno += yyleng; \
}

%}

%option yylineno
%option noyywrap

%x comment

TYPE "int" | "float" | "char"
STRUCT "struct"
IF "if"
ELSE "else"
WHILE "while"
RETURN "return"
DOT "."
SEMI ";"
COMMA ","
ASSIGN "="
LT "<"
LE "<="
GT ">"
GE ">="
NE "!="
EQ "=="
PLUS "+"
MINUS "-"
MUL "*"
DIV "/"
AND "&&"
OR "||"
NOT "!"
LP "("
RP ")"
LB "["
RB "]"
LC "{"
RC "}"

ID [a-zA-Z_]([a-zA-Z0-9])* 
decimal -?(0|[1-9][0-9]*)
hexadecimal -?(0x)[0-9a-fA-F]* 
INT {decimal}|{hexadecimal}
FLOAT [0-9]+.[0-9]+
CHAR \'(\\x[0-9a-fA-F]{2}|.)\' 

INVALID_CHAR \'.*\' 

WRONG_ID ({INT}{ID})|{INVALID_CHAR}

COMMENT "//".*$
MULTI_COMMENT "/*".*?"*/" 

ILLEGAL_TOKEN [^\[[:space:]]]
INVALID_NUMBER ([+-]?0x[0-9a-zA-Z]*)|(00+)

%% 

"//"[^\n]*       {  } 
"/*"             { BEGIN(comment); }
<comment>"*/"    { BEGIN(INITIAL); } 
<comment>.       {  }
<comment>"\n"    { yycolno = 1; }
.                {  } 
"\n"             { yycolno = 1; }

%% 

int main(void ) {
    yylex();     
    // test 
}