# Lüt Compiler Development Plan

This document outlines the plan for evolving the Lüt compiler from its current C-intermediary approach to direct machine code generation using LLVM.

## Current Architecture

```
Lüt Source → Lexer/Parser → AST → BytecodeCompiler → Instructions → C Code → GCC/Clang → Executable
```

## Target Architecture

```
Lüt Source → Lexer/Parser → AST → BytecodeCompiler → Instructions → LLVM IR → LLVM → Executable
```

## Development Phases

### Phase 1: LLVM Integration (Short Term)

#### 1.1. Setup LLVM Bindings
- Add the `inkwell` crate as a dependency (Rust bindings for LLVM)
- Set up the basic LLVM context, module, and builder structure
- Create a simple "Hello World" test to verify LLVM integration

#### 1.2. Develop LLVM Code Generator
- Create a new `LLVMCodeGenerator` struct that will translate Instructions to LLVM IR
- Implement basic translation for fundamental instructions:
  - `LoadNumber`, `LoadText`, `StoreVariable`, `LoadVariable`
  - `PrintValue`, `PrintNewline`
- Keep the existing BytecodeCompiler for generating instructions

#### 1.3. Runtime Environment
- Create LLVM structs that match our ValueType, Value, etc.
- Implement memory management functions in LLVM IR
- Port the stack operations (push/pop) to LLVM

#### 1.4. Basic Operations
- Implement arithmetic operations (Add, Subtract, Multiply, Divide, Modulo)
- Implement the ASCII conversion functionality

#### 1.5. JIT Execution Mode
- Add a JIT execution mode for rapid testing during development
- Allow running programs directly without writing to disk

#### 1.6. Output Executable Generation
- Generate executable files for the current platform
- Set appropriate permissions on Unix systems
- Test with existing example programs

### Phase 2: Feature Parity (Medium Term)

#### 2.1. Full Instruction Support
- Ensure all instructions from the current bytecode VM are supported in the LLVM version
- Add comprehensive tests to verify behavior matches the current implementation

#### 2.2. Optimization Passes
- Implement basic LLVM optimization passes
- Measure and improve performance compared to the C-intermediary approach

#### 2.3. Error Handling
- Improve error messages and error recovery
- Add source location information to generated LLVM IR for better debugging

#### 2.4. Cross-Compilation
- Add support for targeting multiple platforms
- Create a platform specification system in the CLI

### Phase 3: New Features (Long Term)

#### 3.1. Advanced Data Types
- Add support for boolean, array, and map types
- Implement associated operations in LLVM

#### 3.2. Control Flow
- Add support for if/else conditions
- Implement loops (while, for, foreach)
- Translate control flow structures to efficient LLVM IR

#### 3.3. Functions
- Add function definition and calling syntax
- Implement function IR generation including parameter passing and return values

#### 3.4. Standard Library
- Design a small standard library of essential functions
- Implement them in efficient LLVM IR

#### 3.5. Performance Enhancements
- Profile generated code and identify bottlenecks
- Apply LLVM optimizations specific to Lüt code patterns
- Benchmark against other languages

## Implementation Notes

### Compatibility Considerations

1. **Backwards Compatibility**: Ensure all existing Lüt programs continue to work
2. **Runtime Behavior**: Maintain the same runtime behavior as the C implementation
3. **Error Messages**: Keep the same error message format for consistency

### LLVM Integration Strategy

1. Start with the existing `Instruction` enum as intermediate representation
2. Create mapping functions from Instructions to LLVM IR
3. Reuse the bytecode compiler to generate instructions
4. Gradually replace the C backend with direct LLVM IR generation

### Implementation Approach

```rust
// Example skeleton for LLVM code generation

struct LLVMCodeGenerator<'ctx> {
    context: &'ctx Context,
    module: &'ctx Module,
    builder: &'ctx Builder,
    // Function references, type definitions, etc.
    runtime_functions: HashMap<String, FunctionValue<'ctx>>,
    value_struct_type: StructType<'ctx>,
    // Symbol table
    variables: HashMap<String, PointerValue<'ctx>>,
}

impl<'ctx> LLVMCodeGenerator<'ctx> {
    // Initialize the code generator with LLVM context
    fn new(context: &'ctx Context, module: &'ctx Module, builder: &'ctx Builder) -> Self { ... }
    
    // Set up basic types and runtime functions
    fn initialize_runtime(&mut self) -> Result<(), String> { ... }
    
    // Generate LLVM IR from instructions
    fn generate_code(&mut self, instructions: &[Instruction]) -> Result<(), String> { ... }
    
    // Helper functions for various instruction types
    fn compile_load_number(&self, value: i64) -> Result<BasicValueEnum<'ctx>, String> { ... }
    fn compile_load_text(&self, text: &str) -> Result<BasicValueEnum<'ctx>, String> { ... }
    fn compile_binary_op(&self, op: InstructionType) -> Result<(), String> { ... }
    
    // Write the generated LLVM module to an executable file
    fn write_executable(&self, path: &Path) -> Result<(), String> { ... }
}
```

## Milestones & Timeline

### Milestone 1: LLVM Proof of Concept
- Basic LLVM integration
- Simple instruction translation (number literals, text literals, print)
- JIT execution of simple programs

### Milestone 2: Core Feature Implementatio
- Complete instruction translation
- Variable handling
- Arithmetic operations
- Initial executable generation

### Milestone 3: Compiler Maturity
- Full test suite passing
- Optimization passes
- Robust error handling
- Cross-compilation for major platforms

### Milestone 4: Language Evolution
- New language features
- Standard library
- Documentation and examples
- Community engagement

## Resources

- [Inkwell Documentation](https://thedan64.github.io/inkwell/inkwell/index.html)
- [LLVM Language Reference](https://llvm.org/docs/LangRef.html)
- [Kaleidoscope Tutorial](https://llvm.org/docs/tutorial/MyFirstLanguageFrontend/index.html)
- [Writing an LLVM Backend](https://llvm.org/docs/WritingAnLLVMBackend.html)

## Community Contribution Opportunities

- **Beginner Level**: Documentation, tests, simple instruction implementations
- **Intermediate Level**: Control flow structures, error handling improvements
- **Advanced Level**: LLVM optimization passes, advanced data types, cross-compilation