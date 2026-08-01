-- A bit of a failed experiment with open covers. Maybe we're better off just
-- using regular sets instead of trying to define structs.
import Mathlib.Data.SetLike.Basic
import Mathlib.Topology.Defs.Basic

import Munkres.Mathlib.Prelude

namespace Munkres

universe u w

variable
  {α : Type u} [TopologicalSpace α]
  {ι : Type w}

@[ext]
structure iOpenCover (K : Set α) (ι : Type w) where
  toFun : ι → Set α
  isOpen' : ∀ i, IsOpen (toFun i)
  cover' : K ⊆ ⋃ i, toFun i

instance (K : Set α) : FunLike (iOpenCover K ι) ι (Set α) where
  coe := iOpenCover.toFun
  coe_injective' _ _ := iOpenCover.ext

@[ext]
structure sOpenCover (K : Set α) where
  carrier : Set (Set α)
  isOpen' : ∀ u ∈ carrier, IsOpen u
  cover' : K ⊆ ⋃₀ carrier

instance (K : Set α) : SetLike (sOpenCover K) (Set α) where
  coe := sOpenCover.carrier
  coe_injective' _ _ := sOpenCover.ext

-- instance (K : Set α) : HasSubset (sOpenCover K) where
--   Subset a b :=

-- example {K : Set α} : IsCompact K ↔ ∀ (U : sOpenCover K),
--   ∃ F ⊆ U, true
--   := by
--   -- ∃ F : iOpenCover K ι, F := by
--   sorry

end Munkres