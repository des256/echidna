# Simplified Grammar for Encoding and Decoding

Attr = `#` `[` Path [ DelimTokenTree | ( `=` LiteralSuffixlessExpr ) ] `]` .
Visibility = `pub` [ `(` `crate` | `self` | `super` | ( `in` Path ) `)` ] .

GenericArg = ( `'` IDENTIFIER )  | Type | ( IDENTIFIER `=` Type ) | ( Type `as` Path ) .

PathSeg = IDENTIFIER | ( `<` GenericArg { `,` GenericArg } `>` ) .
Path = [ `::` ] PathSeg { `::` PathSeg } .

TupleType = `(` [ Type { `,` Type } [ `,` ] `)` .
ArrayType = [ Type `;` Expr ] .
Type = Path | TupleType | ArrayType .

TraitBound = [ `?` ] [ `for` `<` `'` IDENTIFIER { `,` `'` IDENTIFIER } `>` ] TypePath .
TypeParamBound = ( `'` IDENTIFIER ) | ( `(` TraitBound `)` ) | TraitBound .
TypeParam = IDENTIFIER [ `:` [ TypeParamBound { `+` TypeParamBound } [ `+` ] ] ] [ `=` Type ] .
LifetimeParam = `'` IDENTIFIER [ `:` `'` IDENTIFIER { `+` `'` IDENTIFIER } [ `+` ] ] .
GenericParam = LifetimeParam | TypeParam .
Generics = `<` GenericParam { `,` GenericParam } [ `,` ] `>` .

LifetimeWhere = `'` IDENTIFIER `:` `'` IDENTIFIER { `+` `'` IDENTIFIER } [ `+` ] .
TypeBoundWhere = [ `for` `<` `'` IDENTIFIER { `,` `'` IDENTIFIER } `>` ] Type `:` [ TypeParamBound { `+` TypeParamBound } [ `+` ] ] .
Where = LifetimeWhere | TypeBoundWhere .
WhereClause = `where` { Where `,` } [ Where ] .

StructField = { Attr } [ Visibility ] IDENTIFIER `:` Type .
Struct = `struct` IDENTIFIER [ Generics ] [ WhereClause ] ( `{` [ StructField { `,` StructField } [ `,` ] ] `}` ) | `;` .

TupleField = { Attr } [ Visibility ] Type .
Tuple = `struct` IDENTIFIER [ Generics ] `(` [ TupleField { `,` TupleField } [ `,` ] ] `)` [ WhereClause ] `;` .

EnumItemStruct = `{` [ StructField { `,` StructField } [ `,` ] ] `}` .
EnumItemTuple = `(` [ TupleField { `,` TupleField } [ `,` ] ] `)` .
EnumItemDiscr = `=` Expr .
EnumItem = { Attr } [ Visibility ] IDENTIFIER [ EnumItemStruct | EnumItemTuple | EnumItemDiscr ] .
EnumItems = EnumItem { `,` EnumItem } [ `,` ] .
Enum = `enum` IDENTIFIER [ Generics ] [ WhereClause ] `{` [ EnumItems ] `}` .

Item = { Attr } [ Visibility ] Struct | Tuple | Enum .
