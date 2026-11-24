import Lake

open Lake DSL

package Onigiri where
  version := v!"0.1.0"
  keywords := #["math"]
  leanOptions := #[
    ⟨`pp.unicode.fun, true⟩,
    ⟨`autoImplicit, false⟩,
    ⟨`weak.linter.privateModule, false⟩,
    ⟨`weak.linter.mathlibStandardSet, true⟩,
  ]

require "leanprover-community" / "mathlib" @ git "v4.37.0-rc1"

@[default_target]
lean_lib «Onigiri» where
  -- add any library configuration options here
