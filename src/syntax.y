%{
#include "ast.hh"
void yyerror(const char* s);
int has_error=0;
%}

%locations

%union{
    Any v; 
}

%nonassoc <v> ILLEGAL_TOKEN
%nonassoc LOWER_ELSE
%nonassoc <v> ELSE

%token <v> TYPE INT CHAR FLOAT STRUCT ID  
%token <v> IF WHILE RETURN  

%token <v> COMMA
 
%right <v> ASSIGN

%left <v> OR
%left <v> AND
%left <v> LT LE GT GE NE EQ 
%left <v> PLUS MINUS 
%left <v> MUL DIV 
%right <v> NOT
%left <v> DOT LP RP LB RB 

%token <v> SEMI LC RC /* punctuation word */
%token <v> INVALID_CHAR WRONG_ID UNKNOWN_CHAR INVALID_NUMBER /* mistake word */

/* Non terminal */
%type <v> Program ExtDefList ExtDef ExtDecList
%type <v> Specifier StructSpecifier 
%type <v> VarDec FunDec VarList ParamDec CompSt StmtList
%type <v> Stmt DefList Def DecList Dec Exp Args

%%

Program : ExtDefList { 
    Wrapper program = { $1, @$.first_line }; 
    $$ = std::move(program); 
}
;

ExtDefList : {
    ExtDefList ext; 
    Wrapper extdeflist = { ext, @$.first_line }; 
    $$ = std::move(extdeflist); 
};
| ExtDef ExtDefList {
    ExtDefList ext = { $1, $2 }; 
    Wrapper extdeflist = { ext, @$.first_line }; 
    $$ = std::move(extdeflist); 
}
;