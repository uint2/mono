import Lake

open System Lake DSL

package Munkres where
  version := v!"0.1.0"
  keywords := #["math"]
  leanOptions :=
  #[⟨`pp.unicode.fun, true⟩,
    ⟨`autoImplicit, false⟩,
    ⟨`weak.linter.mathlibStandardSet, true⟩]

require "leanprover-community" / mathlib

@[default_target] lean_lib Munkres
