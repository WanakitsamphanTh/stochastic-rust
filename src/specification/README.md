# Syntax

**specification** : modelType DefineSection? ModelSection InitSection \
**modelType** : `dtmc` | `ctmc` \
**DefineSection** : `define` `:` `\n` [variableDeclaration `\n`]+ `\n` \
**variableDeclaration** : variable `=` expression \
**ModelSection** : `model` `:` `\n` [transitionDeclaration `\n`]+ `\n`
**transitionDeclaration** : identifier [`->` `(` expression `)` identifier]+ \
**InitSection** : `init` `:` `\n` [initialization `\n`]+ `\n` \
**initilization** : identifier `=` expression

**expression**: binary \
**binary**: unary [binaryOp unary]? \
**unary**: [`-`]? primary \
**primary**: grouping | number | variable \
**grouping**: `(` expression `)` \
**number**: `$` [0 | [1-9][0-9]\*][.[1-9]\*]?  \
**variable**: `$` [alphanumeric | _]* \
**identifier**: [alphanumeric | _]*