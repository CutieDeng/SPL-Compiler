#pragma once 

#include <optional> 
#include <any> 

using MyStr = char const * ;
using Any = std::any; 
using Opt = std::optional<Any>;

struct Args {
    exp: Any, 
    args: Opt, 
};

struct BiExp {
    first: Any, 
    second: Any, 
    // I bet it as a string, 
    operator: Any, 
};

struct SingleExp {
    exp: Any, 
};

struct ExpValue {
    exp: Any, 
}; 

struct UnaryExp {
    exp: Any, 
    operator : Any, 
}; 

struct CallExp {
    id: Any, 
    args: Opt,
}; 

struct IdxExp {
    exp: Any, 
    idx: Any, 
}; 

struct FieldExp {
    exp: Any, 
    field: Any, 
}; 

struct Dec {
    var: Any, 
    exp: Opt, 
};

struct DecList {
    dec: Any, 
    dec_list: Opt, 
};

struct Def {
    specifier: Any, 
    dec_list: Any, 
}; 

struct DefList{
    def: Opt, 
    def_list: Opt, 
};

struct ExpStmt {
    exp: Any, 
};

struct CompStmt {
    def_list: Any, 
    stmt_list: Any, 
};

struct StmtList {
    stmt: Opt, 
    stmt_list: Opt, 
}; 

struct ReturnStmt {
    exp: Any, 
}; 

struct IfStmt {
    condition_exp: Any, 
    if_stmt: Any, 
    else_stmt: Opt, 
}; 

struct WhileStmt {
    exp: Any, 
    stmt: Any, 
}; 

struct ParamDec {
    specifier: Any, 
    var_dec: Any, 
}; 

struct VarList {
    param_dec: Any, 
    var_list: Opt, 
};

struct FunDec {
    id: Any, 
    var_list: Opt, 
}; 

struct VarDecId {
    id: Any, 
};  

struct VarDec {
    var_dec: Any, 
    // Int 
    i: Any, 
}; 

struct SpecifierType {
    type: Any, 
}; 

struct StructSpecifier {
    id: Any, 
    def_list: Opt, 
}; 

struct SpecifierStruct {
    struct_specifier: Any, 
}; 

struct ExtDef {
    specifier: Any, 
    ext_decl_list: Opt, 
    fun_dec: Opt, 
    comp_stat: Opt, 
}; 

struct ExtDecList {
    var_dec: Any, 
    ext_dec_list: Opt, 
}; 

struct ExtDefList {
    ext_def : Opt, 
    list: Opt, 
}; 

struct Program {
    ext_def_list: Any, 
}; 

struct Wrapper {
    content: Any, 
    line: long, 
}; 

void print(Any const &node, int indent = 0) {
    return ; 
}