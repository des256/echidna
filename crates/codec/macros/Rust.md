# Rust Grammar

## Lexical Structure

SimplePath = [ `::` ] SimplePathSegment { `::` SimplePathSegment } .

SimplePathSegment = IDENTIFIER | `super` | `self` | `crate` | `$crate` .

PathInExpression = [ `::` ] PathExprSegment { `::` PathExprSegment } .

PathExprSegment = PathIdentSegment [ `::` GenericArgs ] .

PathIdentSegment = IDENTIFIER | `super` | `self` | `Self` | `crate` | `$crate` .

GenericArgs = `<` [ GenericArgsLifetimes ] [ GenericArgsTypes ] [ GenericArgsBindings ] `>` .

GenericArgsLifetimes = Lifetime { `,` Lifetime } [ `,` ] .

GenericArgsTypes = Type { `,` Type } [ `,` ] .

GenericArgsBindings = GenericArgsBinding { `,` GenericArgsBinding } .

GenericArgsBinding = IDENTIFIER `=` Type .

QualifiedPathInExpression = QualifiedPathType `::` PathExprSegment { `::` PathExprSegment } .

QualifiedPathType = `<` Type [ `as` TypePath ] `>` .

QualifiedPathInType = QualifiedPathType `::` TypePathSegment { `::` TypePathSegment } .

TypePath = [ `::` ] TypePathSegment { `::` TypePathSegment } .

TypePathSegment = PathIdentSegment [ `::` ] [ GenericArgs | TypePathFn ] .

TypePathFn = `(` [ TypePathFnInputs ] `)` [ `->` Type ] .

TypePathFnInputs = Type { `,` Type } [ `,` ] .

## Macros

MacroInvocation = SimplePath `!` DelimTokenTree .

DelimTokenTree = ( `(` { TokenTree } `)` ) | ( `[` { TokenTree } `]` ) | ( `{` { TokenTree } `}` ) .

TokenTree = NonDelimToken | DelimTokenTree .

MacroInvocationSemi = ( SimplePath `!` `(` { TokenTree } `)` `;` ) | ( SimplePath `!` `[` { TokenTree } `]` `;` ) | ( SimplePath `!` `{` { TokenTree } `}` ) .

MacroRulesDefinition = `macro_rules` `!` IDENTIFIER MacroRulesDef .

MacroRulesDef = ( `(` MacroRules `)` `;` ) | ( `[` MacroRules `]` `;` ) | ( `{` MacroRules `}` ) .

MacroRules = MacroRule { `;` MacroRule } [ `;` ] .

MacroRule = MacroMatcher `=>` MacroTranscriber .

MacroMatcher = ( `(` MacroMatch `)` ) | ( `[` MacroMatch `]` ) | ( `{` MacroMatch `}` ) .

MacroMatch = NonDelimNonDollarToken | MacroMatcher | ( `$` IDENTIFIER `:` MacroFragSpec ) | ( `$` `(` MacroMatch { MacroMatch } `)` [ MacroRepSep ] MacroRepOp ) .

MacroFragSpec = `block` | `expr` | `ident` | `item` | `lifetime` | `literal` | `meta` | `pat` | `path` | `stmt` | `tt` | `ty` | `vis` .

MacroRepSep = NonDelimNonRepToken .

MacroRepOp = `*` | `+` | `?` .

MacroTranscriber = DelimTokenTree .

## Crates and Source Files

Crate = [ UTF8BOM ] [ SHEBANG ] { InnerAttribute } { Item } .

UTF8BOM = `\uFEFF` .

SHEBANG = `#!` ~ `\n` .

## Conditional Compilation

ConfigurationPredicate = ConfigurationOption | ConfigurationAll | ConfigurationAny | ConfigurationNot .

ConfigurationOption = IDENTIFIER [ `=` STRING_LITERAL | RAW_STRING_LITERAL ] .

ConfigurationAll = `all` `(` [ ConfigurationPredicateList ] `)` .

ConfigurationAny = `any` `(` [ ConfigurationPredicateList ] `)` .

ConfigurationNot = `not` `(` ConfigurationPredicateList `)` .

ConfigurationPredicateList = ConfigurationPredicate { `,` ConfigurationPredicate } [ `,` ] .

CfgAttribute = `cfg` `(` ConfigurationPredicate `)` .

CfgAttrAttribute = `cfg_attr` `(` ConfigurationPredicate `,` [ CfgAttrs ] `)` .

CfgAttrs = Attr { `,` Attr } [ `,` ] .

## Items

Item = { OuterAttribute } VisItem | MacroItem .

VisItem = [ Visibility ] Module | ExternCrate | UseDeclaration | Function | TypeAlias | Struct | Enumeration | Union | ConstantItem | StaticItem | Trait | Implementation | ExternBlock .

MacroItem = MacroInvocationSemi | MacroRulesDefinition .

Module = [ `unsafe` ] `mod` IDENTIFIER `;` | ( `{` { InnerAttribute } { Item } `}` ) .

ExternCrate = `extern` `crate` CrateRef [ AsClause ] `;` .

CrateRef = IDENTIFIER | `Self` .

AsClause = `as` IDENTIFIER | `_` .

UseDeclaration = `use` UseTree `;` .

UseTree = ( [ [ SimplePath ] `::` ] `*` ) | ( [ [ SimplePath ] `::` ] `{` [ UseTree { `,` UseTree } [ `,` ] ] `}` ) | ( SimplePath [ `as` IDENTIFIER | `_` ] ) .

Function = FunctionQualifiers `fn` IDENTIFIER [ Generics ] `(` [ FunctionParameters ] `)` [ FunctionReturnType ] [ WhereClause ] BlockExpression .

FunctionQualifiers = [ AsyncQualifiers ] [ `unsafe` ] [ `extern` [ Abi ] ] .

AsyncConstQualifiers = `async` | `const` .

Abi = STRING_LITERAL | RAW_STRING_LITERAL .

FunctionParameters = FunctionParam { `,` FunctionParam } [ `,` ] .

Functionparam = { OuterAttribute } Pattern `:` Type .

FunctionReturnType = `->` Type .

TypeAlias = `type` IDENTIFIER [ Generics ] [ WhereClause ] `=` Type `;` .

Struct = StructStruct | TupleStruct .

StructStruct = `struct` IDENTIFIER [ Generics ] [ WhereClause ] ( `{` [ StructFields ] `}` ) | `;` .

TupleStruct = `struct` IDENTIFIER [ Generics ] `(` [ TupleFields ] `)` [ WhereClause ] `;` .

StructFields = StructField { `,` StructField } [ `,` ] .

StructField = { OuterAttribute } [ Visibility ] IDENTIFIER `:` Type .

TupleFields = TupleField { `,` TupleField } [ `,` ] .

TupleField = { OuterAttribute } [ Visibility ] Type .

Enumeration = `enum` IDENTIFIER [ Generics ] [ WhereClause ] `{` [ EnumItems ] `}` .

EnumItems = EnumItem { `,` EnumItem } [ `,` ] .

EnumItem = { OuterAttribute } [ Visibility ] IDENTIFIER [ EnumItemTuple | EnumItemStruct | EnumItemDiscriminant ] .

EnumItemTuple = `(` [ TupleFields ] `)` .

EnumItemStruct = `{` [ StructFields ] `}` .

EnumItemDiscriminant = `=` Expression .

Union = `union` IDENTIFIER [ Generics ] [ WhereClause ] `{` StructFields `}` .

ConstantItem = `const` IDENTIFIER | `_` `:` Type `=` Expression `;` .

StaticItem = `static` [ `mut` ] IDENTIFIER `:` Type `=` Expression `;` .

Trait = `trait` IDENTIFIER [ Generics ] [ `:` [ TypeParamBounds ] ] [ WhereClause ] `{` { InnerAttribute } { TraitItem } `}` .

TraitItem = { OuterAttribute } [ Visibility ] TraitFunc | TraitMethod | TraitConst | TraitType | MacroInvocationSemi .

TraitFunc = TraitFunctionDecl `;` | BlockExpression .

TraitMethod = TraitMethodDecl `;` | BlockExpression .

TraitFunctionDecl = FunctionQualifiers `fn` IDENTIFIER [ Generics ] `(` [ TraitFunctionParameters ] `)` [ FunctionReturnType ] [ WhereClause ] .

TraitMethodDecl = FunctionQualifiers `fn` IDENTIFIER [ Generics ] `(` SelfParam { `,` TraitFunctionparam } [ `,` ] `)` [ FunctionReturnType ] [ WhereClause ] .

TraitFunctionParameters = TraitFunctionParam { `,` TraitFunctionparam } [ `,` ] .

TraitFunctionparam = { OuterAttribute } [ Pattern `:` ] Type .

TraitConst = `const` IDENTIFIER `:` Type [ `=` Expression ] `;` .

TraitType = IDENTIFIER [ `:` [ TypeParamBounds ] ] `;` .

Implementation = InherentImpl | TraitImpl .

InherentImpl = `impl` [ Generics ] Type [ WhereClause ] `{` { InnerAttribute } { InherentImplItem } `}` .

InherentImplItem = { OuterAttribute } MacroInvocationSemi | ( [ Visibility ] ConstantItem | Function | Method ) .

TraitImpl = [ `unsafe` ] `impl` [ Generics ] [ `!` ] TypePath `for` Type [ WhereClause ] `{` { InnerAttribute } { TraitImplItem } `}` .

TraitImplItem = { OuterAttribute } MacroInvocationSemi | ( [ Visibility ] TypeAlias | ConstantItem | Function | Method ) .

ExternBlock = [ `unsafe` ] `extern` [ Abi ] `{` { InnerAttribute } { ExternalItem } `}` .

ExternalItem = { OuterAttribute } MacroInvocationSemi | ( [ Visibility ] ExternalStaticItem | ExternalFunctionItem ) .

ExternalStaticItem = `static` [ `mut` ] IDENTIFIER `:` Type `;` .

ExternalFunctionItem = `fn` IDENTIFIER [ Generics ] `(` [ NamedFunctionParameters | NamedFunctionParametersWithVariadics ] `)` [ FunctionReturnType ] [ WhereClause ] .

NamedFunctionParameters = NamedFunctionParam { `,` NamedFunctionParam } [ `,` ] .

NamedFunctionParam = { OuterAttribute } IDENTIFIER | `_` `:` Type .

NamedFunctionParametersWithVariadics = { NamedFunctionParam `,` } NamedFunctionParam `,` { OuterAttribute } `...` .

Generics = `<` GenericParams `>` .

GenericParams = [ LifetimeParams ] [ `,` TypeParams ] [ `,` ].

LifetimeParams = LifetimeParam { `,` LifetimeParam } .

LifetimeParam = [ OuterAttribute ] LIFETIME_OR_LABEL [ `:` LifetimeBounds ] .

TypeParams = TypeParam { `,` TypeParam } .

TypeParam = [ OuterAttribute ] IDENTIFIER [ `:` [ TypeParamBounds ] ] [ `=` Type ] .

WhereClause = `where` { WhereClauseItem `,` } [ WhereClauseItem ] .

WhereClauseItem = LifetimeWhereClauseItem | TypeBoundWhereClauseItem .

LifetimeWhereClauseItem = Lifetime `:` LifetimeBounds .

TypeBoundWhereClauseItem = [ ForLifetimes ] Type `:` [ TypeParamBounds ] .

ForLifetimes = `for` `<` LifetimeParams `>` .

Method = FunctionQualifiers `fn` IDENTIFIER [ Generics ] `(` SelfParam { `,` FunctionParam } [ `,` ] ) [ FunctionReturnType ] [ WhereClause ] BlockExpression .

SelfParam = { OuterAttribute } ShorthandSelf | TypedSelf .

ShorthandSelf = [ `&` | ( `&` Lifetime ) ] [ `mut` ] `self` .

TypedSelf = [ `mut` ] `self` `:` Type .

Visibility = `pub` [ `(` `crate` | `self` | `super` | ( `in` SimplePath ) `)` ] .

## Attributes

InnerAttribute = `#` `!` `[` Attr `]` .

OuterAttribute = `#` `[` Attr `]` .

Attr = SimplePath [ AttrInput ] .

AttrInput = DelimTokenTree | ( `=` LiteralSuffixlessExpression ) .

MetaItem = SimplePath [ ( `=` LiteralSuffixlessExpression ) | ( `(` MetaSeq `)` ) ] .

MetaSeq = MetaItemInner { `,` MetaItemInner } [ `,` ] .

MetaItemInner = MetaItem | LiteralSuffixlessExpression .

MetaWord = IDENTIFIER .

MetaNameValueStr = IDENTIFIER STRING_LITERAL | RAW_STRING_LITERAL .

MetaListPaths = IDENTIFIER `(` [ SimplePath { `,` SimplePath } [ `,` ] ] `)` .

MetaListIdents = IDENTIFIER `(` [ IDENTIFIER { `,` IDENTIFIER } [ `,` ] ] `)` .

MetaListNameValueStr = IDENTIFIER `(` MetaNameValueStr { `,` MetaNameValueStr } [ `,` ] `)` .

## Statements and Expressions

Statement = `;` | Item | LetStatement | ExpressionStatement | MacroInvocationSemi .

LetStatement = { OuterAttribute } `let` Pattern [ `:` Type ] [ `=` Expression ] `;` .

ExpressionStatement = ExpressionWithoutBlock | ExpressionWithBlock [ `;` ] .

ExpressionWithoutBlock = { OuterAttribute } LiteralExpression | PathExpression | OperatorExpression | GroupedExpression | ArrayExpression | AwaitExpression | IndexExpression | TupleExpression | TupleIndexingExpression | StructExpression | EnumerationVariantExpression | CallExpression | MethodCallExpression | FieldExpression | ClosureExpression | ContinueExpression | BreakExpression | RangeExpression | ReturnExpression | MacroInvocation .

ExpressionWithBlock = { OuterAttribute } BlockExpression | AsyncBlockExpression | UnsafeBlockExpression | LoopExpression | IfExpression | IfLetExpression | MatchExpression .

LiteralExpression = CHAR_LITERAL | STRING_LITERAL | RAW_STRING_LITERAL | BYTE_LITERAL | BYTE_STRING_LITERAL | RAW_BYTE_STRING_LITERAL | INTEGER_LITERAL | FLOAT_LITERAL | BOOLEAN_LITERAL .

PathExpression = PathInExpression | QualifiedPathInExpression .

BlockExpression = `{` { InnerAttribute } [ Statements ] `}` .

Statements = ( Statement { Statement } [ ExpressionWithoutBlock ] ) | ExpressionWithoutBlock .

AsyncBlockExpression = `async` [ `move` ] BlockExpression .

UnsafeBlockExpression = `unsafe` BlockExpression .

OperatorExpression = BorrowExpression | DereferenceExpression | ErrorPropagationExpression | NegationExpression | ArithmeticOrLogicalExpression | ComparisonExpression | LazyBooleanExpression | TypeCastExpression | AssignmentExpression | CompoundAssignmentExpression .

BorrowExpression = `&` | `&&` [ `mut` ] Expression .

DereferenceExpression = `*` Expression .

ErrorPropagationExpression = Expression `?` .

NegationExpression = `-` | `!` Expression .

ArithmeticOrLogicalExpression = Expression `+` | `-` | `*` | `/` | `%` | `&` | `|` | `^` | `<<` | `>>` Expression .

ComparisonExpression = Expression `==` | `!=` | `>` | `<` | `>=` | `<=` Expression .

LazyBooleanExpression = Expression `||` | `&&` Expression .

TypeCastExpression = Expression `as` TypeNoBounds .

AssignmentExpression = Expression `=` Expression .

CompoundAssignmentExpression = Expression `+=` | `-=` | `*=` | `/=` | `%=` | `&=` | `|=` | `^=` | `<<=` | `>>=` Expression .

GroupedExpression = `(` { InnerAttribute } Expression `)` .

ArrayExpression = `[` { InnerAttribute } [ ArrayElements ] `]` .

ArrayElements = ( Expression { `,` Expression } [ `,` ] ) | ( Expression `;` Expression ) .

IndexExpression = Expression `[` Expression `]` .

TupleExpression = `(` { InnerAttribute } [ TupleElements ] `)` .

TupleElements = Expression { `,` Expression } [ `,` ] .

TupleIndexingExpression = Expression `.` TUPLE_INDEX .

StructExpression = StructExprStruct | StructExprTuple | StructExprUnit .

StructExprStruct = PathInExpression `{` { InnerAttribute } [ StructExprFields | StructBase ] `}` .

StructExprFields = StructExprField { `,` StructExprField } [ `,` StructBase [ `,` ] ] .

StructExprField = IDENTIFIER | ( IDENTIFIER | TUPLE_INDEX `:` Expression ) .

StructBase = `..` Expression .

StructExprTuple = PathInExpression `(` { InnerAttribute } [ Expression { `,` Expression } [ `,` ] ] `)` .

StructExprUnit = PathInExpression .

EnumVariantExpression = EnumExprStruct | EnumExprTuple | EnumExprFieldless .

EnumExprStruct = PathInExpression `{` [ EnumExprFields ] `}` .

EnumExprFields = EnumExprField { `,` EnumExprField } [ `,` ] .

EnumExprField = IDENTIFIER | ( IDENTIFIER | TUPLE_INDEX `:` Expression ) .

EnumExprTuple = PathInExpression `(` [ Expression { `,` Expression } [ `,` ] ] `)` .

EnumExprFieldless = PatthInExpression .

CallExpression = Expression `(` [ CallParams ] `)` .

CallParams = Expression { `,` Expression } [ `,` ] .

MethodCallExpression = Expression `.` PathExprSegment `(` [ CallParams ] `)` .

FieldExpression = Expression `.` IDENTIFIER .

ClosureExpression = [ `move` ] `||` | ( `|` [ ClosureParameters ] `|` ) Expression | ( `->` TypeNoBounds BlockExpression ) .

ClosureParameters = ClosureParam { `,` ClosureParam } [ `,` ] .

ClosureParam = { OuterAttribute } Pattern [ `:` Type ] .

LoopExpression = [ LoopLabel ] InfiniteLoopExpression | PredicateLoopExpression | PredicatePatternLoopExpression | IteratorLoopExpression .

InfiniteLoopExpression = `loop` BlockExpression .

PredicateLoopExpression = `while` StructlessExpression BlockExpression .

PredicatePatternLoopExpression = `while` `let` MatchArmsPatterns `=` StructlessLazylessExpression BlockExpression .

IteratorLoopExpression = `for` Pattern `in` StructlessExpression BlockExpression .

LoopLabel = LIFETIME_OR_LABEL `:` .

BreakExpression = `break` [ LIFETIME_OR_LABEL ] [ Expression ] .

ContinueExpression = `continue` [ LIFETIME_OR_LABEL ] .

RangeExpression = RangeExpr | RangeFromExpr | RangeToExpr | RangeFullExpr | RangeInclusiveExpr | RangeToInclusiveExpr .

RangeExpr = Expression `..` Expression .

RangeFromExpr = Expression `..` .

RangeToExpr = `..` Expression .

RangeFullExpression = `..` .

RangeInclusiveExpression = Expression `..=` Expression .

RangeToInclusiveExpression = `..=` Expression .

IfExpression = `if` StructlessExpression BlockExpression [ `else` BlockExpression | IfExpression | IfLetExpression ] .

IfLetExpression = `if` `let` MatchArmPatterns `=` StructlessLazylessExpression BlockExpression [ `else` BlockExpression | IfExpression | IfLetExpression ] .

MatchExpression = `match` StructlessExpression `{` { InnerAttribute } [ MatchArms ] `}` .

MatchArms = { MatchArm `=>` ( ExpressionWithoutBlock `,` ) | ( ExpressionWithBlock `,` ) } MatchArm `=>` Expression [ `,` ] .

MatchArm = { OuterAttribute } MatchArmPatterns [ MatchArmGuard ] .

MatchArmPatterns = [ `|` ] Pattern { `|` Pattern } .

MatchArmGuard = `if` Expression .

ReturnExpression = `return` [ Expression ] .

AwaitExpression = Expression `.` `await` .

## Patterns

Pattern = PatternWithoutRange | RangePattern .

PatternWithoutRange = LiteralPattern | IdentifierPattern | WildcardPattern | RestPattern | ObsoleteRangePattern | ReferencePattern | StructPattern | TupleStructPattern | TuplePattern | GroupedPattern | SlicePattern | PathPattern | MacroInvocation .

LiteralPattern = BOOLEAN_LITERAL | CHAR_LITERAL | BYTE_LITERAL | STRING_LITERAL | RAW_STRING_LITERAL | BYTE_STRING_LITERAL | RAW_BYTE_STRING_LITERAL | ( [ `-` ] INTEGER_LITERAL | FLOAT_LITERAL ) .

IdentifierPattern = [ `ref` ] [ `mut` ] IDENTIFIER [ `@` Pattern ] .

WildcardPattern = `_` .

RestPattern = `..` .

RangePattern = RangePatternBound `..=` RangePatternBound .

ObsoleteRangePattern = RangePatternBound `...` RangePatternBound .

RangePatternBound = CHAR_LITERAL | BYTE_LITERAL | PathInExpression | QualifiedPathInExpression ( [ `-` ] INTEGER_LITERAL | FLOAT_LITERAL ) .

ReferencePattern = `&` | `&&` [ `mut` ] PatternWithoutRange .

StructPattern = PathInExpression `{` [ StructPatternElements ] `}` .

StructPatternElements = [ StructPatternFields ] [ `,` StructPatternEtc ] .

StructPatternFields = StructPatternField { `,` StructPatternField } .

StructPatternField = { OuterAttribute } ( TUPLE_INDEX `:` Pattern ) | ( IDENTIFIER `:` Pattern ) | ( [ `ref` ] [ `mut` ] IDENTIFIER ) .

StructPatternEtc = { OuterAttribute } `..` .

TupleStructPattern = PathInExpression `(` [ TupleStructItems ] `)` .

TupleStructItems = Pattern { `,` Pattern } [ `,` ] .

TuplePattern = `(` [ TuplePatternItems ] `)` .

TuplePatternItems = ( Pattern `,` ) | RestPattern | ( Pattern `,` Pattern { `,` Pattern } [ `,` ] ) .

GroupedPattern = `(` Pattern `)` .

SlicePattern = `[` [ SlicePatternItems ] `]` .

SlicePatternItems = Pattern { `,` Pattern } [ `,` ] .

PathPattern = PathInExpression | QualifiedPathInExpression .

## Type System

Type = TypeNoBounds | ImplTraitType | TraitObjectType .

TypeNoBounds = ParenthesizedType | ImplTraitTypeOneBound | TraitObjectTypeOneBound | TypePath | TupleType | NeverType | RawPointerType | ReferenceType | ArrayType | SliceType | InferredType | QualifiedPathInType | BareFunctionType | MacroInvocation .

ParenthesizedType = `(` Type `)` .

NeverType = `!` .

TupleType = `(` [ Type { `,` Type } [ `,` ] `)` .

ArrayType = [ Type `;` Expression ] .

SliceType = `[` Type `]` .

ReferenceType = `&` [ Lifetime ] [ `mut` ] TypeNoBounds .

RawPointerType = `*` `mut` | `const` TypeNoBounds .

BareFunctionType = [ ForLifetimes ] FunctionQualifiers `fn` `(` [ FunctionParametersMaybeNamedVariadic ] `)` [ BareFunctionReturnType ] .

BareFunctionReturnType = `->` TypeNoBounds .

FunctionParametersMaybeNamedVariadic = MaybeNamedFunctionParameters | MaybeNamedFunctionParametersVariadic .

MaybeNamedFunctionParameters = MaybeNamedParam { `,` MaybeNamedParam } [ `,` ] .

MaybeNamedParam = { OuterAttribute } [ IDENTIFIER | `_` `:` ] Type .

MaybeNamedFunctionParametersVariadic = { MaybeNamedParam `,` } MaybeNamedParam `,` { OuterAttribute } `...` .

TraitObjectType = [ `dyn` ] TypeParamBounds .

TraitObjectTypeOneBound = [ `dyn` ] TraitBound .

ImplTraitType = `impl` TypeParamBounds .

ImplTraitTypeOneBound = `impl` TraitBound .

InferredType = `_` .

TypeParamBounds = TypeParamBound { `+` TypeParamBound } [ `+` ] .

TypeParamBound = Lifetime | TraitBound .

TraitBound = ( [ `?` ] [ ForLifetimes ] TypePath ) | ( `(` [ `?` ] [ ForLifetimes ] TypePath `)` ) .

LifetimeBounds = Lifetime { `+` Lifetime } [ `+` ] .

Lifetime = LIFETIME_OR_LABEL | `'static` | `'_` .
