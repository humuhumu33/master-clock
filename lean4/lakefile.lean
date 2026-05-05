import Lake
open Lake DSL

package «PrimeFoundations» where
  leanOptions := #[
    ⟨`pp.unicode.fun, true⟩,
    ⟨`autoImplicit, false⟩
  ]

@[default_target]
lean_lib «PrimeFoundations» where
  globs := #[.submodules `PrimeFoundations]

require mathlib from git
  "https://github.com/leanprover-community/mathlib4.git"@"v4.23.0"
