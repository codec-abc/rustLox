pushd %~dp0



cargo run .\examples\HelloWorld.lox
cargo run .\examples\blocks\MultipleNestedBlocks.lox
@REM cargo run .\examples\class\CallInheritedMethod.lox
@REM cargo run .\examples\class\ClassMethodCall.lox
@REM cargo run .\examples\class\ClassWithInit.lox
@REM cargo run .\examples\class\ComplicatedThisResolution.lox
@REM cargo run .\examples\class\InstanceCanAccessItsStateFromItsMethods.lox
@REM cargo run .\examples\class\InstanceWithProperties.lox
@REM cargo run .\examples\class\OverrideMethod.lox
@REM cargo run .\examples\class\PrintClass.lox
@REM cargo run .\examples\class\PrintInstance.lox
@REM cargo run .\examples\class\PrintThis.lox
@REM cargo run .\examples\class\SuperLookupStartsInClassContainingSuper.lox
@REM cargo run .\examples\functions\Fibonacci.lox
@REM cargo run .\examples\functions\FunctionsCloseOverFreeVariablesCorrectly.lox
@REM cargo run .\examples\functions\FunctionWithReturn.lox
@REM cargo run .\examples\functions\NestedFunctions.lox
@REM cargo run .\examples\functions\PrintFunction.lox
@REM cargo run .\examples\functions\PrintResultOfFunctionWithoutReturn.lox
@REM cargo run .\examples\functions\RecursiveFunction.lox
@REM cargo run .\examples\functions\ReturnFromNestedBlocks.lox
cargo run .\examples\logical\operators\IfOperator.lox
cargo run .\examples\logical\operators\LogicalOperators.lox
cargo run .\examples\loops\ForLoop.lox
cargo run .\examples\loops\WhileLoop.lox
cargo run .\examples\statements\SimplePrintStatements.lox
cargo run .\examples\variables\GlobalVariable.lox
cargo run .\examples\variables\MultipleVariables.lox
cargo run .\examples\variables\RedefineGlobalVariable.lox
cargo run .\examples\variables\UninitializedVariable.lox
popd