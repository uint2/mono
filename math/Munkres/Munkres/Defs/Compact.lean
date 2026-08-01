import Mathlib.Topology.Neighborhoods

import Munkres.Defs.Basic

namespace Munkres

open Topology Filter

universe u

variable {α : Type u}

-- Equivalence of the idea of convergence. WOH.
example [TopologicalSpace α] (f : ℕ → α) (x : α)
  : Tendsto f atTop (𝓝 x) ↔ ∀ U, x ∈ U → IsOpen U → ∃ N, ∀ k ≥ N, f k ∈ U
  := by --
  rw [tendsto_atTop_nhds] -- ∎

/-- A topological space is `limit point compact` if every infinite subset of X
has a limit point in X. -/
def LimitPointCompact (α : Type u) [TopologicalSpace α] : Prop :=
  ∀ s : Set α, s.Infinite → ∃ x, AccPt x (𝓟 s)

--* Munkres Theorem 29.2
/-- Munkres calls this `Locally Compact Spaces`. Mathlib, calls it
`WeaklyLocallyCompactSpace`. -/
protected theorem WeaklyLocallyCompactSpace.iff [TopologicalSpace α]
  : WeaklyLocallyCompactSpace α ↔ ∀ x : α, ∃ c, ∃ u ∈ nhds' x, u ⊆ c ∧ IsCompact c
  := by --
  constructor
  · intro h x
    obtain ⟨c, hc, hcx⟩ := h.exists_compact_mem_nhds x
    rw [mem_nhds_iff] at hcx
    obtain ⟨u, huc, hu, hxu⟩ := hcx
    exact ⟨c, u, ⟨hu, hxu⟩, huc, hc⟩
  · intro h
    refine {exists_compact_mem_nhds := ?_}
    intro x
    specialize h x
    obtain ⟨c, u, ⟨hu, hxu⟩, huc, hc⟩ := h
    simp only [mem_nhds_iff]
    exact ⟨c, hc, u, huc, hu, hxu⟩ -- ∎

end Munkres
